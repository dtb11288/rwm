use crate::layout::Layout;
use crate::window::Window;
use log::{debug};

pub struct Workspace<W> {
    name: String,
    layout: Box<dyn Layout<W>>,
    pub windows: Vec<Window<W>>,
}

impl<W> Workspace<W> {
    pub fn new(name: String, windows: Vec<Window<W>>, layout: Box<dyn Layout<W>>) -> Self {
        let mut workspace = Self { name, windows, layout };
        workspace.perform_layout()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_window(mut self, window: Window<W>) -> Self {
        self.windows.push(window);
        self.perform_layout()
    }

    pub fn change_layout<L: Layout<W> + 'static>(mut self, layout: L) -> Self {
        self.layout = Box::new(layout);
        self.perform_layout()
    }

    fn perform_layout(self) -> Self {
        debug!("Handle layout {} for workspace {}", &self.name, &self.layout.get_name());
        let handled_windows = self.layout.handle_layout(self.windows);
        Self {
            windows: handled_windows,
            ..self
        }
    }
}
