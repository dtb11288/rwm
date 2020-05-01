pub mod xcb_server;

#[derive(Clone)]
pub enum DisplayType {
    Xcb
}

impl From<&str> for DisplayType {
    fn from(display: &str) -> Self {
        match display {
            "xcb" => DisplayType::Xcb,
            _ => panic!("Invalid display server {}", display)
        }
    }
}
