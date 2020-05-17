use crate::window::{Window, Geometry};
use crate::layouts::Layout;
use crate::stack::Stack;

#[derive(Clone)]
pub struct FullScreen;

impl Layout for FullScreen {
    fn handle_layout(&self, _view: &Geometry, _windows: Stack<Window>) -> Stack<Window> {
        unimplemented!()
    }
}
