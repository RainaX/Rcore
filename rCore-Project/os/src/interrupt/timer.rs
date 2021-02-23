use crate::sbi::set_timer;
use riscv::register::{time, sie};

pub static mut TICKS: usize = 0;

static INTERVAL: usize = 100000;

pub fn init () {
    unsafe {
        sie::set_stimer();
    }

    set_next_timeout();
}


fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        // if TICKS % 100 == 0 {
        //     println!("{} tick", TICKS);
        // }
    }
}

