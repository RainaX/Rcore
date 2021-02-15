use super::VectorAllocator;
use bit_field::BitArray;
use core::cmp::max;


const MIN_ALIGN_BITMAP_SIZE: usize = 8192;

pub struct BuddySystemVectorAllocator {
    min_align: usize,
    capacity: usize,
    bitmap: [u8; MIN_ALIGN_BITMAP_SIZE / 4],
}


impl VectorAllocator for BuddySystemVectorAllocator {
    fn new(capacity: usize) -> Self {
        let mut bitmap = [0xff; MIN_ALIGN_BITMAP_SIZE / 4];
        bitmap.set_bit(0, false);

        if capacity <= MIN_ALIGN_BITMAP_SIZE {
            let mut actual_capacity = MIN_ALIGN_BITMAP_SIZE;
            while actual_capacity > capacity {
                actual_capacity >>= 2;
            }

            Self {
                min_align: 1,
                capacity: actual_capacity,
                bitmap,
            }
        } else {
            let mut min_align_shift = 0;

            while (capacity >> min_align_shift) >= MIN_ALIGN_BITMAP_SIZE {
                min_align_shift += 1;
            }
            min_align_shift -= 1;

            Self {
                min_align: 1 << min_align_shift,
                capacity: MIN_ALIGN_BITMAP_SIZE,
                bitmap,
            }
        }
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        if align.count_ones() != 1 {
            return None;
        }

        let mut actual_size = size / self.min_align;
        let mut actual_align = max(1, align / self.min_align);
        if size % self.min_align != 0 {
            actual_size += 1;
        }
        if actual_size > self.capacity || actual_align > self.capacity {
            return None;
        }

        while actual_align < actual_size {
            actual_align <<= 1;
        }

        
        let mut base = 0;
        let mut len = 1;
        let mut curr_align = self.capacity;

        while curr_align > actual_align {
            base += len;
            len <<= 1;
            curr_align >>= 1;
        }

        let mut idx = base;
        loop {
            for _ in 0..len {
                if !self.bitmap.get_bit(idx) {
                    break;
                }
                idx += 1;
            }
            if idx < base + len {
                break;
            }

            if base == 0 {
                return None;
            }
 
            len >>= 1;
            base -= len;
            curr_align <<= 1;
            idx = base;
        }

        while curr_align > actual_align {
            self.bitmap.set_bit(idx, true);
            self.bitmap.set_bit(idx * 2 + 2, false);

            idx = idx * 2 + 1;
            curr_align >>= 1;
            base += len;
            len <<= 1;
        }

        self.bitmap.set_bit(idx, true);
        Some((idx - base) * actual_align * self.min_align)
    }

    fn dealloc(&mut self, start: usize, size: usize, align: usize) {
        let mut actual_size = size / self.min_align;
        let mut actual_align = max(1, align / self.min_align);
        if size % self.min_align != 0 {
            actual_size += 1;
        }

        while actual_align < actual_size {
            actual_align <<= 1;
        }

        let mut base = 0;
        let mut len = 1;
        let mut curr_align = self.capacity;

        while curr_align > actual_align {
            base += len;
            len <<= 1;
            curr_align >>= 1;
        }

        let mut idx = base + start / actual_align / self.min_align;
        self.bitmap.set_bit(idx, false);

        while idx > 0 {
            let sibling = if idx % 2 == 1 {
                idx + 1
            } else {
                idx - 1
            };
            if self.bitmap.get_bit(sibling) {
                break;
            }

            self.bitmap.set_bit(idx, true);
            self.bitmap.set_bit(sibling, true);

            idx = (idx - 1) / 2;
            self.bitmap.set_bit(idx, false);
        }
    }
}  
