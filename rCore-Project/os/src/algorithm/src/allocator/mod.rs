mod stacked_allocator;
mod segment_tree_allocator;

mod bitmap_vector_allocator;


pub trait Allocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, index: usize);
}


pub trait VectorAllocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use stacked_allocator::StackedAllocator;
pub use segment_tree_allocator::SegmentTreeAllocator;

pub use bitmap_vector_allocator::BitmapVectorAllocator;

pub type AllocatorImpl = SegmentTreeAllocator;

pub type VectorAllocatorImpl = BitmapVectorAllocator;
