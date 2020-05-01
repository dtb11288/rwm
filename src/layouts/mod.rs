use crate::layout::Layout;
use crate::layouts::tall::Tall;
use crate::layouts::fullscreen::FullScreen;

pub mod fullscreen;
pub mod tall;

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutType {
    Tall,
    FullScreen,
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

impl From<LayoutType> for Box<dyn Layout> {
    fn from(layout_type: LayoutType) -> Self {
        match layout_type {
            LayoutType::Tall => Box::new(Tall),
            LayoutType::FullScreen => Box::new(FullScreen),
        }
    }
}
