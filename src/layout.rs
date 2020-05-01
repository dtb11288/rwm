use crate::window::{Window, Geometry};
use std::fmt;
use crate::layouts::LayoutType;

impl PartialEq for dyn Layout {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl fmt::Debug for dyn Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{:?}|", self.get_name())
    }
}

pub trait Layout {
    fn get_name(&self) -> LayoutType;
    fn handle_layout(&self, workspace_view: &Geometry, windows: Vec<Window>) -> Vec<Window>;
}
