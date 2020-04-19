#[derive(Clone)]
pub struct Config {
    pub tags: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        Self {
            tags: vec!["1", "2", "3"].into_iter().map(String::from).collect(),
        }
    }
}
