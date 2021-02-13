use super::*;
use crate::memory::*;
use lazy_static::*;
use spin::Mutex;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(*KERNEL_END_ADDRESS)..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}

pub struct FrameAllocator {
    top_header_addr: PhysicalAddress,
}

#[derive(Copy, Clone)]
struct FrameHeader {
    count: usize,
    next_header_addr: PhysicalAddress,
}

impl FrameAllocator {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        let allocator = FrameAllocator {
            top_header_addr: range.into().start.into(),
        };

        let top_header = FrameHeader {
            count: range.into().len(),
            next_header_addr: PhysicalAddress(0),
        };

        unsafe {
            *(allocator.top_header_addr.0 as *mut FrameHeader) = top_header;
        }

        allocator
    }

    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        if self.top_header_addr.0 == 0 {
            return MemoryResult::Err("no available frame to allocate");
        }

        let mut alloc_page_number: PhysicalPageNumber = self.top_header_addr.into();

        unsafe {
            let mut top_header = *(self.top_header_addr.0 as *mut FrameHeader);

            if top_header.count > 1 {
                top_header.count -= 1;
                alloc_page_number += top_header.count;
                *(self.top_header_addr.0 as *mut FrameHeader) = top_header;
            } else {
                self.top_header_addr = top_header.next_header_addr;
            }
        }

        Ok(FrameTracker(alloc_page_number))
    }

    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        let new_top_header = FrameHeader {
            count: 1,
            next_header_addr: self.top_header_addr,
        };

        unsafe {
            *(frame.address().0 as *mut FrameHeader) = new_top_header;
        }

        self.top_header_addr = frame.address();
    }
}
