use crate::window::{Window, View};

pub trait Layout<W: Clone + PartialEq> {
    fn get_name(&self) -> &str;
    fn handle_layout(&self, workspace_view: &View, windows: Vec<Window<W>>) -> Vec<Window<W>>;
}
