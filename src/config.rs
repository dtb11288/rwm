use crate::layouts::Layout;
use crate::keys::ModKey;

#[derive(Clone)]
pub struct Config {
    pub mod_key: ModKey,
    pub workspaces: Vec<String>,
    pub layouts: Vec<Layout>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            mod_key: ModKey::Mod4,
            layouts: vec!["tall".into(), "fullscreen".into()],
            workspaces: vec!["1", "2", "3"].into_iter().map(String::from).collect(),
        }
    }
}
