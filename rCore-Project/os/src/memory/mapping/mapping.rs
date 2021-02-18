use crate::memory::{
    address::*,
    config::PAGE_SIZE,
    frame::{FrameTracker, FRAME_ALLOCATOR},
    mapping::{Flags, MapType, PageTable, PageTableEntry, PageTableTracker, Segment},
    MemoryResult,
};
use alloc::{collections::VecDeque, vec, vec::Vec};
use core::cmp::min;
use core::ptr::slice_from_raw_parts_mut;


#[derive(Default)]
pub struct Mapping {
    page_tables: Vec<PageTableTracker>,
    root_ppn: PhysicalPageNumber,
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}


impl Mapping {
    pub fn activate(&self) {
        let new_satp = self.root_ppn.0 | (8 << 60);
        unsafe {
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }


    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }


    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, Some(vpn.into()), segment.flags | Flags::VALID)?;
                }

                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }

            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    let mut page_data = [0u8; PAGE_SIZE];
                    if let Some(init_data) = init_data {
                        if !init_data.is_empty() {
                            let page_address = VirtualAddress::from(vpn);
                            let start = if segment.range.start > page_address {
                                segment.range.start - page_address
                            } else {
                                0
                            };
                            let stop = min(PAGE_SIZE, segment.range.end - page_address);
                            let dst_slice = &mut page_data[start..stop];
                            let src_slice = &init_data[(page_address + start - segment.range.start)
                                ..(page_address + stop - segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    };

                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    (*frame).copy_from_slice(&page_data);
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }

        Ok(())
    }


    pub fn unmap(&mut self, segment: &Segment) {
        for vpn in segment.page_range().iter() {
            let entry = self.find_entry(vpn).unwrap();
            assert!(!entry.is_empty());
            entry.clear();
        }

        self.mapped_pairs.retain(|(vpn, _)| !segment.page_range().contains(*vpn));
        }


    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                self.page_tables.push(new_table);
            }
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        Ok(entry)
    }


    pub fn lookup(va: VirtualAddress) -> Option<PhysicalAddress> {
        let mut current_ppn;
        unsafe {
            llvm_asm!("csrr $0, satp" : "=r"(current_ppn) ::: "volatile");
            current_ppn ^= 8 << 60;
        }

        let root_table: &PageTable = 
            PhysicalAddress::from(PhysicalPageNumber(current_ppn)).deref_kernel();
        let vpn = VirtualPageNumber::floor(va);
        let mut entry = &root_table.entries[vpn.levels()[0]];
        let mut length = 12 + 2 * 9;
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                return None;
            }
            if entry.has_next_level() {
                length -= 9;
                entry = &mut entry.get_next_table().entries[*vpn_slice];
            } else {
                break;
            }
        }
        let base = PhysicalAddress::from(entry.page_number()).0;
        let offset = va.0 & ((1 << length) - 1);
        Some(PhysicalAddress(base + offset))
    }


    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()> {
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");

        *entry = PageTableEntry::new(ppn, flags);
        Ok(())
    }
}

