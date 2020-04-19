use crate::config::Config;
use crate::window::{Window, WindowType, View};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Event<W: Debug + PartialEq> {
    KeyPressed(String),
    WindowAdded(W, WindowType),
    WindowRemoved(W),
    WindowFocused(W),
    Ignored
}

pub trait DisplayServer: Iterator<Item=Event<<Self as DisplayServer>::Window>>{
    type Window: Debug + Clone + PartialEq;
    fn new(config: &Config) -> Self;
    fn get_root_view(&self) -> View;
    fn configure_window(&self, window: &Window<Self::Window>);
    fn close_window(&self, window: &Window<Self::Window>);
}
