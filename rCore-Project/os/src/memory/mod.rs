#![allow(dead_code)]


pub mod address;
pub mod config;
pub mod heap;
pub mod frame;
pub mod range;

pub type MemoryResult<T> = Result<T, &'static str>;

pub use {address::*, config::*, frame::FRAME_ALLOCATOR, range::Range};


pub fn init() {
    heap::init();
    unsafe {
        riscv::register::sstatus::set_sum()
    };

    println!("mod memory initialized");
}
