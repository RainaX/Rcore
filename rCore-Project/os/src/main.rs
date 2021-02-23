#![no_std]

#![no_main]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod drivers;
mod fs;
mod interrupt;
mod kernel;
mod memory;
mod panic;
mod process;
mod sbi;


global_asm!(include_str!("entry.asm"));

extern crate alloc;

use alloc::sync::Arc;
use fs::{INodeExt, ROOT_INODE};
use memory::PhysicalAddress;
use process::*;
use xmas_elf::ElfFile;


#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    interrupt::init();
    memory::init();
    drivers::init(dtb_pa);
    fs::init();
    
    {
        let mut processor = PROCESSOR.lock();

        let kernel_process = Process::new_kernel().unwrap();

        
        for i in 1..2usize {
            processor.add_thread(create_kernel_thread(
                kernel_process.clone(),
                sample_process as usize,
                Some(&[i]),
            ));
        }
    }

    extern "C" {
        fn __restore(context: usize);
    }

    let context = PROCESSOR.lock().prepare_next_thread();
    unsafe { __restore(context as usize) };

    unreachable!()
}


fn sample_process(id: usize) {
    println!("hello from kernel thread {}", id);
    let magic_number = 10086;
    loop {
        for _ in 0..100_0000 {}
        let id = PROCESSOR.lock().current_thread().id;
        println!("Thread ID: {}", id);
        println!("Magic Number: {}", magic_number);
    }
}

pub fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    let thread = Thread::new(process, entry_point, arguments).unwrap();

    thread
        .as_ref()
        .inner()
        .context
        .as_mut()
        .unwrap()
        .set_ra(kernel_thread_exit as usize);

    thread
}


fn kernel_thread_exit() {
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;

    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}
