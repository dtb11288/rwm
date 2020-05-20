use crate::window::{Window, Geometry};
use crate::layouts::Layout;
use crate::stack::Stack;

#[derive(Clone)]
pub struct FullScreen;

impl Layout for FullScreen {
    fn handle_layout(&self, view: &Geometry, windows: Stack<Window>) -> Stack<Window> {
        windows.into_iter()
            .map(|(is_current, window)| {
                (is_current, window.set_view(view.clone()).visible(is_current))
            })
            .collect()
    }
}
