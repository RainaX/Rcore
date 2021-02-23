use crate::memory::{
    address::*,
    config::*,
    mapping::{Flags, MapType, Mapping, Segment},
    range::Range,
    MemoryResult,
};
use alloc::{vec, vec::Vec};
use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};


pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
}


impl MemorySet {
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }


        let segments = vec![
            Segment {
                map_type: MapType::Linear,
                range: Range::from(DEVICE_START_ADDRESS..DEVICE_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },

            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },

            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },

            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },

            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },

            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;

        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet {
            mapping,
            segments,
        })
    }

    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<MemorySet> {
        let mut memory_set = MemorySet::new_kernel()?;

        for program_header in file.program_iter() {
            if program_header.get_type() != Ok(Type::Load) {
                continue;
            }

            let start = VirtualAddress(program_header.virtual_addr() as usize);
            let size = program_header.mem_size() as usize;
            let data: &[u8] =
                if let SegmentData::Undefined(data) = program_header.get_data(file).unwrap() {
                    data
                } else {
                    return Err("unsupported elf format");
                };

            let segment = Segment {
                map_type: MapType::Framed,
                range: Range::from(start..(start + size)),
                flags: Flags::user(is_user)
                    | Flags::readable(program_header.flags().is_read())
                    | Flags::writable(program_header.flags().is_write())
                    | Flags::executable(program_header.flags().is_execute()),
            };

            memory_set.add_segment(segment, Some(data))?;
        }

        Ok(memory_set)
    }

    pub fn fork(&self) -> MemoryResult<MemorySet> {
        let mut child_memory_set = MemorySet::new_kernel()?;

        for parent_seg in self.segments.iter() {
            if child_memory_set.overlap_with(parent_seg.page_range()) {
                continue;
            }
        
            for vpn in parent_seg.page_range().iter() {
                let child_seg = Segment {
                    range: (vpn..(vpn + 1)).into(),
                    ..*parent_seg
                };
                let data: &[u8; PAGE_SIZE] = VirtualAddress::from(vpn).deref();
                child_memory_set.add_segment(child_seg, Some(data))?;
            } 
        }
        Ok(child_memory_set)
    }


    pub fn activate(&self) {
        self.mapping.activate();
    }


    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        assert!(!self.overlap_with(segment.page_range()));
        self.mapping.map(&segment, init_data)?;
        self.segments.push(segment);
        Ok(())
    }


    pub fn remove_segment(&mut self, segment: &Segment) -> MemoryResult<()> {
        let segment_index = self
            .segments
            .iter()
            .position(|s| s == segment)
            .expect("segment to remove cannot be found");
        self.segments.remove(segment_index);
        self.mapping.unmap(segment);
        Ok(())
    }


    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        for seg in self.segments.iter() {
            if range.overlap_with(&seg.page_range()) {
                return true;
            }
        }
        false
    }
}


