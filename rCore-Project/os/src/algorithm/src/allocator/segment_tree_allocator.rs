use super::Allocator;
use alloc::{vec, vec::Vec};
use bit_field::BitField;

pub struct SegmentTreeAllocator {
    seg_tree: Vec<u8>,
}

fn get_bit(v: &Vec<u8>, bit: usize) -> bool {
    v[bit / 8].get_bit(bit % 8)
}

fn set_bit(v: &mut Vec<u8>, bit: usize, value: bool) {
    let mut b = v[bit / 8];
    b.set_bit(bit % 8, value);
    v[bit / 8] = b;
}


impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        let mut tree_size = 1;

        while tree_size < capacity {
            tree_size *= 2
        }
        tree_size *= 2;

        let mut seg_tree = vec![0; tree_size / 8];

        for idx in 0..capacity {
            set_bit(&mut seg_tree, tree_size / 2 - 1 + idx, true);
        }

        for idx in (0..(tree_size / 2 - 1)).rev() {
            let value = get_bit(&seg_tree, idx * 2 + 1) || get_bit(&seg_tree, idx * 2 + 2);
            set_bit(&mut seg_tree, idx, value);
        }


        Self {
            seg_tree,
        }
    }

    
    fn alloc(&mut self) -> Option<usize> {
        if !get_bit(&self.seg_tree, 0) {
            None
        } else {
            let mut idx = 0;

            let tree_size = self.seg_tree.len();
            while idx < tree_size / 2 - 1 {
                if get_bit(&self.seg_tree, idx * 2 + 1) {
                    idx = idx * 2 + 1;
                } else {
                    idx = idx * 2 + 2;
                }
            }

            let offset = idx + 1 - tree_size / 2;

            set_bit(&mut self.seg_tree, idx, false);
            while idx > 0 {
                idx = (idx - 1) / 2;
                let value = get_bit(&self.seg_tree, idx * 2 + 1) || get_bit(&self.seg_tree, idx * 2 + 2);
                set_bit(&mut self.seg_tree, idx, value);
            }            

            Some(offset)
        } 
    }


    fn dealloc(&mut self, index: usize) {
        let tree_size = self.seg_tree.len();
        let mut idx = index + tree_size / 2 - 1;

        set_bit(&mut self.seg_tree, idx, true);
        while idx > 0 {
            idx = (idx - 1) / 2;
            set_bit(&mut self.seg_tree, idx, true);
        }
    }
}
