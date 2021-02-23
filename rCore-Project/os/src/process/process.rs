use super::*;
use crate::fs::*;
use xmas_elf::ElfFile;


pub struct Process {
    pub is_user: bool,
    pub inner: Mutex<ProcessInner>,
}


pub struct ProcessInner {
    pub memory_set: MemorySet,
    pub descriptors: Vec<Arc<dyn INode>>,
}

impl Process {
    pub fn new_kernel() -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            is_user: false,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::new_kernel()?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        }))
    }

    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            is_user,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::from_elf(file, is_user)?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        }))
    }

    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }

    pub fn alloc_page_range(
        &self,
        size: usize,
        flags: Flags,
    ) -> MemoryResult<Range<VirtualAddress>> {
        let memory_set = &mut self.inner().memory_set;

        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }

        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(self.is_user),
            },
            None,
        )?;

        Ok(Range::from(range.start..(range.start + size)))
    }
}
