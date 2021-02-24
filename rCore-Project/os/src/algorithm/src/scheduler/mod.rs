mod fifo_scheduler;
mod hrrn_scheduler;
mod stride_scheduler;


pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    type Priority;

    fn add_thread(&mut self, thread: ThreadType);

    fn get_next(&mut self) -> Option<ThreadType>;

    fn remove_thread(&mut self, thread: &ThreadType);

    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority);
}

pub use fifo_scheduler::FifoScheduler;
pub use hrrn_scheduler::HrrnScheduler;
pub use stride_scheduler::StrideScheduler;

pub type SchedulerImpl<T> = StrideScheduler<T>;
