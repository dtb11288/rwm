use crate::window::{Window, Geometry};
use crate::stack::Stack;
use std::fmt::Debug;

mod fullscreen;
mod tall;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Layout {
    Tall,
    FullScreen,
}

impl Layout {
    pub fn handle_layout<W>(&self, view: &Geometry, windows: Stack<Window<W>>) -> Stack<Window<W>> {
        match self {
            Layout::Tall => tall::handle_layout(view, windows),
            Layout::FullScreen => fullscreen::handle_layout(view, windows),
        }
    }
}

impl From<&str> for Layout {
    fn from(display: &str) -> Self {
        match display {
            "tall" => Layout::Tall,
            "fullscreen" => Layout::FullScreen,
            _ => panic!("Invalid layout {}", display)
        }
    }
}
