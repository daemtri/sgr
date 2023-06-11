use std::thread;

pub trait Runable {
    fn run(&self);
}

pub struct Engine {}

impl Engine {
    fn new() -> Self {
        Engine {}
    }
    fn run(&self, runables: Vec<Box<dyn Runable + Send>>) {
        for n in runables {
            thread::spawn(move || {
                n.run();
            })
            .join()
            .unwrap();
        }
    }
}
