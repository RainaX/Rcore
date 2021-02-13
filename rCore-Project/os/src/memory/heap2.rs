use super::config::KERNEL_HEAP_SIZE;
use algorithm::{VectorAllocator, VectorAllocatorImpl};
use core::cell::UnsafeCell;

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[global_allocator]
static HEAP: Heap = Heap(UnsafeCell::new(None));

struct Heap(UnsafeCell<Option<VectorAllocatorImpl>>);


unsafe impl alloc::alloc::GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let offset = (*self.0.get())
            .as_mut()
            .unwrap()
            .alloc(layout.size(), layout.align())
            .expect("Heap overflow");
        &mut HEAP_SPACE[offset] as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let offset = ptr as usize - &HEAP_SPACE as *const _ as usize;
        (*self.0.get())
            .as_mut()
            .unwrap()
            .dealloc(offset, layout.size(), layout.align());
    }
}

unsafe impl Sync for Heap {}

pub fn init() {
    unsafe {
        (*HEAP.0.get()).replace(VectorAllocatorImpl::new(KERNEL_HEAP_SIZE));
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}
