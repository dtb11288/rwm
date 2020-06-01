use crate::window::{Window, Geometry};
use crate::stack::Stack;

pub fn handle_layout<W>(view: &Geometry, windows: Stack<Window<W>>) -> Stack<Window<W>> {
    windows.into_iter()
        .map(|(is_current, window)| {
            (is_current, window.set_view(view.clone()).visible(is_current))
        })
        .collect()
}
