use super::timer;
use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::*;

global_asm!(include_str!("./interrupt.asm"));


pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }

        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}


#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        Trap::Exception(Exception::LoadFault) => load_fault(context, stval),
        _ => fault(context, scause, stval),
    }
}

fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}


fn supervisor_timer(_context: &mut Context) {
    timer::tick();
}

fn load_fault(_context: &mut Context, stval: usize) {
    if stval == 0x0 {
        println!("SUCCESS!");
    }
    panic!("Load fault");
}


fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
} 

