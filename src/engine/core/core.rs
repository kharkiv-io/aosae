use std::thread;

pub enum Groups {
    Priority,
    Default,
    Freeze,
    Killed,
}

pub fn init_core() {}

pub fn core_loop() {
    loop {
        let i = thread::spawn(move || {});
    }
}
