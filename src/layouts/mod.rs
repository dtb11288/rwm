use crate::layouts::tall::Tall;
use crate::layouts::fullscreen::FullScreen;
use crate::window::{Window, Geometry};
use crate::stack::Stack;

mod fullscreen;
mod tall;

trait Layout {
    fn handle_layout(&self, view: &Geometry, windows: Stack<Window>) -> Stack<Window>;
}

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutType {
    Tall,
    FullScreen,
}

impl LayoutType {
    pub fn handle_layout(&self, view: &Geometry, windows: Stack<Window>) -> Stack<Window> {
        let layout: &dyn Layout = self.into();
        Layout::handle_layout(layout, view, windows)
    }
}

impl From<&str> for LayoutType {
    fn from(display: &str) -> Self {
        match display {
            "tall" => LayoutType::Tall,
            "fullscreen" => LayoutType::FullScreen,
            _ => panic!("Invalid layout {}", display)
        }
    }
}

impl From<&LayoutType> for &dyn Layout {
    fn from(layout_type: &LayoutType) -> Self {
        match layout_type {
            LayoutType::Tall => &Tall,
            LayoutType::FullScreen => &FullScreen,
        }
    }
}
