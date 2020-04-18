use crate::config::Config;
pub mod xcb_server;

#[derive(Debug)]
pub enum DisplayEvent {
    KeyPress(String),
    DoNothing
}

pub trait DisplayServer {
    fn new(config: Config) -> Self;
    fn run<F: Fn(DisplayEvent)>(&self, handler: F);
}
