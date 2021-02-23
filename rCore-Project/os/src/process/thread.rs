use super::*;
use core::hash::{Hash, Hasher};

pub type ThreadID = isize;

static mut THREAD_COUNTER: ThreadID = 0;


pub struct Thread {
    pub id: ThreadID,
    pub stack: Range<VirtualAddress>,
    pub process: Arc<Process>,
    pub inner: Mutex<ThreadInner>,
}


pub struct ThreadInner {
    pub context: Option<Context>,
    pub sleeping: bool,
    pub dead: bool,
}

impl Thread {
    pub fn prepare(&self) -> *mut Context {
        self.process.inner().memory_set.activate();
        let parked_frame = self.inner().context.take().unwrap();
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }

    pub fn park(&self, context: Context) {
        assert!(self.inner().context.is_none());
        self.inner().context.replace(context);
    }

    pub fn new(
        process: Arc<Process>,
        entry_point: usize,
        arguments: Option<&[usize]>,
    ) -> MemoryResult<Arc<Thread>> {
        let stack = process.alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;
        let context = Context::new(stack.end.into(), entry_point, arguments, process.is_user);

        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process,
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
                dead: false,
            }),
        });

        Ok(thread)
    }

    pub fn inner(&self) -> spin::MutexGuard<ThreadInner> {
        self.inner.lock()
    }
}


impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl Eq for Thread {}

impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}


impl core::fmt::Debug for Thread {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Thread")
            .field("thread_id", &self.id)
            .field("stack", &self.stack)
            .field("context", &self.inner().context)
            .finish()
    }
}

