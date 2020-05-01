use crate::window::{Window, Geometry};
use crate::layout::Layout;
use crate::layouts::LayoutType;

#[derive(Clone)]
pub struct FullScreen;

impl Layout for FullScreen {
    fn get_name(&self) -> LayoutType {
        LayoutType::FullScreen
    }

    fn handle_layout(&self, workspace_view: &Geometry, windows: Vec<Window>) -> Vec<Window> {
        unimplemented!()
    }
}
