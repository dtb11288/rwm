use crate::layout::Layout;
use crate::window::{Window, View};

pub struct Tall {}

impl<W> Layout<W> for Tall {
    fn get_name(&self) -> &str {
        "tall"
    }

    fn handle_layout(&self, windows: Vec<Window<W>>) -> Vec<Window<W>> {
        windows.into_iter()
            .map(|window| {
                let view = View::default();
                window.set_view(view)
            })
            .collect()
    }
}
