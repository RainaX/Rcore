use super::Scheduler;
use alloc::collections::LinkedList;

pub struct FifoScheduler<ThreadType: Clone + Eq> {
    pool: LinkedList<ThreadType>,
}


impl<ThreadType: Clone + Eq> Default for FifoScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: LinkedList::new(),
        }
    }
}


impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for FifoScheduler<ThreadType> {
    type Priority = ();
    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.push_back(thread);
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(thread) = self.pool.pop_front() {
            self.pool.push_back(thread.clone());
            Some(thread)
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        let mut removed = self.pool.drain_filter(|t| t == thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    fn set_priority(&mut self, _thread: ThreadType, _priority: ()) {}
}
