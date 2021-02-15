#![no_std]

#![no_main]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;


global_asm!(include_str!("entry.asm"));

extern crate alloc;


#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello rCore-Tutorial!");

    interrupt::init();
    memory::init();

    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }

    println!("heap test passed");
    

    panic!() 

}
