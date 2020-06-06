use crate::window::Geometry;

pub struct Screen<W> {
    window: W,
    workspace: usize,
    view: Geometry,
}

impl<W> Screen<W> {
    pub fn new(window: W, view: Geometry) -> Self {
        Screen {
            window,
            workspace: 0,
            view
        }
    }

    pub fn get_view(&self) -> &Geometry {
        &self.view
    }
}
