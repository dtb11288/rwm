use crate::config::Config;
use crate::window::{Window, WindowType};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Event<W: Debug> {
    KeyPressed(String),
    WindowAdded(W, WindowType),
    WindowRemoved(W),
    WindowFocused(W),
    Ignored
}

pub trait DisplayServer {
    type Event;
    type Window: Debug;
    fn new(config: Config) -> Self;
    fn run<F>(&self, handler: F) where F: Fn(Self::Event);
    fn match_event(&self, event: Self::Event) -> Event<Self::Window>;
    fn configure_window(&self, window: &Window<Self::Window>);
    fn close_window(&self, window: &Window<Self::Window>);
}
