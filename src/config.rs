use crate::layouts::LayoutType;
use crate::displays::DisplayType;
use crate::keys::ModKey;

#[derive(Clone)]
pub struct Config {
    pub mod_key: ModKey,
    pub display: DisplayType,
    pub workspaces: Vec<String>,
    pub layouts: Vec<LayoutType>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            mod_key: ModKey::Mod4,
            layouts: vec!["tall".into(), "fullscreen".into()],
            display: "xcb".into(),
            workspaces: vec!["1", "2", "3"].into_iter().map(String::from).collect(),
        }
    }
}
