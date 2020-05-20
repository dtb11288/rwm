use crate::window::{Window, WindowType, Geometry, WindowId};
use crate::config::Config;
use std::fmt::Debug;

pub mod xcb_server;

#[derive(Clone)]
pub enum DisplayType {
    Xcb
}

impl DisplayType {
    pub fn init(&self, config: &Config) -> Box<dyn DisplayServer> {
        match self {
            DisplayType::Xcb => Box::new(crate::displays::xcb_server::XcbDisplayServer::new(config)),
        }
    }
}

impl From<&str> for DisplayType {
    fn from(display: &str) -> Self {
        match display {
            "xcb" => DisplayType::Xcb,
            _ => panic!("Invalid display server {}", display)
        }
    }
}

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
    fn set_visibility(&self, window: &WindowId, show: bool);
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
