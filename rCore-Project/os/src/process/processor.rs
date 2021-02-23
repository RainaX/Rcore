use super::*;
use algorithm::*;
use hashbrown::HashSet;
use lazy_static::*;


lazy_static! {
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}


lazy_static! {
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        wait_for_interrupt as usize,
        None,
    ).unwrap();
}


unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}


#[derive(Default)]
pub struct Processor {
    current_thread: Option<Arc<Thread>>,
    scheduler: SchedulerImpl<Arc<Thread>>,
    sleeping_threads: HashSet<Arc<Thread>>,
}


impl Processor {
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }

    pub fn prepare_next_thread(&mut self) -> *mut Context {
        if let Some(next_thread) = self.scheduler.get_next() {
            let context = next_thread.prepare();
            self.current_thread = Some(next_thread);
            context
        } else {
            if self.sleeping_threads.is_empty() {
                panic!("all threads terminated, shutting down");
            } else {
                self.current_thread = Some(IDLE_THREAD.clone());
                IDLE_THREAD.prepare()
            }
        }
    }

    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        self.scheduler.add_thread(thread);
    }

    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        thread.inner().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread);
    }

    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().park(*context);
    }

    pub fn sleep_current_thread(&mut self) {
        let current_thread = self.current_thread();

        current_thread.inner().sleeping = true;

        self.scheduler.remove_thread(&current_thread);
        self.sleeping_threads.insert(current_thread);
    }

    pub fn kill_current_thread(&mut self) {
        let thread = self.current_thread.take().unwrap();
        self.scheduler.remove_thread(&thread);
    }
}

