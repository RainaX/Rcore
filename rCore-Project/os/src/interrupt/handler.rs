use super::timer;
use super::context::Context;
use crate::fs::STDIN;
use crate::kernel::syscall_handler;
use crate::memory::*;
use crate::process::PROCESSOR;
use crate::sbi::console_getchar;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

global_asm!(include_str!("./interrupt.asm"));


pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }

        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        sie::set_sext();


        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
        *PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
        *PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
    }
}


#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    {
        let mut processor = PROCESSOR.lock();
        let current_thread = processor.current_thread();
        if current_thread.as_ref().inner().dead {
            println!("thread {} exit", current_thread.id);
            processor.kill_current_thread();
            return processor.prepare_next_thread();
        }
    }

    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        _ => fault("unimplemented interrupt type", scause, stval),
    }
}

fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}


fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().prepare_next_thread()
}


fn supervisor_external(context: &mut Context) -> *mut Context {
    let mut c = console_getchar();
    if c <= 255 {
        if c == '\r' as usize {
            c = '\n' as usize;
        }
        STDIN.push(c as u8);
    }
    context
}

fn fault(msg: &str, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:#x?} terminated: {}",
        PROCESSOR.lock().current_thread(),
        msg
    );
    println!("cause: {:?}, stval: {:x}", scause.cause(), stval);

    PROCESSOR.lock().kill_current_thread();
    PROCESSOR.lock().prepare_next_thread()
} 

