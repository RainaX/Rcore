mod handler;
mod timer;
mod context;

pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
