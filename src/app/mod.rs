mod app;
mod component;

pub use app::*;

pub fn run() {
    App::default().run();
}
