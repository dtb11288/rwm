use crate::layouts::LayoutType;
use crate::displays::DisplayType;

#[derive(Clone)]
pub struct Config {
    pub display: DisplayType,
    pub workspaces: Vec<String>,
    pub layouts: Vec<LayoutType>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            layouts: vec!["tall".into(), "fullscreen".into()],
            display: "xcb".into(),
            workspaces: vec!["1", "2", "3"].into_iter().map(String::from).collect(),
        }
    }
}
