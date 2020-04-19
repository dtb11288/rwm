use crate::layout::Layout;
use crate::window::{Window, View};
use log::{debug};

pub struct Workspace<W: Clone + PartialEq> {
    name: String,
    layout: Box<dyn Layout<W>>,
    is_changed: bool,
    pub view: View,
    pub windows: Vec<Window<W>>,
}

impl<W: Clone + PartialEq> Workspace<W> {
    pub fn new(name: String, windows: Vec<Window<W>>, layout: Box<dyn Layout<W>>, view: View) -> Self {
        let workspace = Self { name, windows, layout, is_changed: false, view };
        workspace.perform_layout()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn reset(mut self) -> Self {
        self.is_changed = false;
        self
    }

    pub fn need_update(mut self) -> Self {
        self.is_changed = true;
        self
    }

    pub fn add_window(mut self, window: Window<W>) -> Self {
        if self.windows.iter().find(|&w| w == &window).is_none() {
            self.windows.push(window);
            self.perform_layout()
        } else {
            self
        }
    }

    pub fn remove_window(mut self, window: W) -> Self {
        let position = self.windows.iter().position(|w| w.get_id() == &window);
        if let Some(position) = position {
            self.windows.remove(position);
            self.perform_layout()
        } else {
            self
        }
    }

    pub fn change_layout<L: Layout<W> + 'static>(mut self, layout: L) -> Self {
        self.layout = Box::new(layout);
        self.perform_layout()
    }

    fn perform_layout(self) -> Self {
        debug!("Handle layout {} for workspace {}", &self.name, &self.layout.get_name());
        let handled_windows = self.layout.handle_layout(&self.view, self.windows);
        Self {
            windows: handled_windows,
            is_changed: true,
            ..self
        }
    }
}
