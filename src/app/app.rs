#[derive(Default, Debug)]
pub struct App {}

impl App {
    pub fn run(&mut self) {
        println!("Hello from App!");
    }
    pub fn with_option<T>(&mut self, x: T) {}
}
