mod handler;
mod timer;
mod context;

pub use context::Context;

pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
