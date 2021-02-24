use super::Scheduler;
use alloc::collections::BTreeSet;
use core::cmp::Ordering;
use bit_field::BitField;

const STRIDE1: usize = 1 << 20;
static mut MIN_PASS: usize = 0;
static mut THREAD_COUNT: usize = 0;

#[derive(Eq)]
struct StrideThread<ThreadType: Clone + Eq> {
    tickets: usize,
    stride: usize,
    pass: usize,
    id: usize,
    pub thread: ThreadType,
}

impl<ThreadType: Clone + Eq> Ord for StrideThread<ThreadType> {
    fn cmp(&self, other: &Self) -> Ordering {
        let signed_pass0 = self.pass.get_bits(0..usize::BIT_LENGTH) as isize;
        let signed_pass1 = other.pass.get_bits(0..usize::BIT_LENGTH) as isize;
        let min;
        unsafe {
            min = MIN_PASS.get_bits(0..usize::BIT_LENGTH) as isize;
        }
        if signed_pass0 != signed_pass1 {
            signed_pass0.overflowing_sub(min).0
                .cmp(&signed_pass1.overflowing_sub(min).0)
        } else {
            self.id.cmp(&other.id)
        }
    }
}

impl<ThreadType: Clone + Eq> PartialOrd for StrideThread<ThreadType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<ThreadType: Clone + Eq> PartialEq for StrideThread<ThreadType> {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass && self.id == other.id
    }
}





pub struct StrideScheduler<ThreadType: Clone + Eq> {
    pool: BTreeSet<StrideThread<ThreadType>>,
}

impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: BTreeSet::new(),
        }
    }
}


impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    type Priority = usize;

    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.insert(StrideThread {
            tickets: 0,
            stride: STRIDE1,
            pass: STRIDE1,
            id: unsafe {
                THREAD_COUNT += 1;
                THREAD_COUNT
            },
            thread,
        });

        unsafe {
            MIN_PASS = self.pool.first().unwrap().pass;
        }
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(mut next) = self.pool.pop_first() {
            let thread = next.thread.clone();
            next.pass = next.pass.overflowing_add(next.stride).0;
            self.pool.insert(next);
            unsafe {
                MIN_PASS = self.pool.first().unwrap().pass;
            }
            Some(thread)
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        let mut removed = self.pool.drain_filter(|t| t.thread == *thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
        let tickets = core::cmp::min(priority, STRIDE1 - 1);
        let stride = STRIDE1 / (tickets + 1);
        
        self.remove_thread(&thread);
        self.pool.insert(StrideThread {
            tickets,
            stride,
            pass: stride,
            id: unsafe {
                THREAD_COUNT += 1;
                THREAD_COUNT
            },
            thread,
        });
        unsafe {
            MIN_PASS = self.pool.first().unwrap().pass;
        }
    }
}
