use crate::window::Window;

pub trait Layout<W> {
    fn get_name(&self) -> &str;
    fn handle_layout(&self, windows: Vec<Window<W>>) -> Vec<Window<W>>;
}
