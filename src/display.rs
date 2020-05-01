use crate::window::{Window, WindowType, Geometry, WindowId};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Event {
    KeyPressed(String),
    WindowAdded(WindowId, WindowType),
    WindowRemoved(WindowId),
    WindowFocused(WindowId),
    Ignored
}

pub trait DisplayServer: Iterator<Item=Event> + CloneServer {
    fn get_root_view(&self) -> Geometry;
    fn configure_window(&self, window: &Window);
    fn close_window(&self, window: &Window);
}

pub trait CloneServer {
    fn clone_server(&self) -> Box<dyn DisplayServer>;
}

impl<D> CloneServer for D where D: DisplayServer + Clone + 'static {
    fn clone_server(&self) -> Box<dyn DisplayServer> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DisplayServer> {
    fn clone(&self) -> Self {
        self.clone_server()
    }
}
