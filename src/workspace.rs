use crate::layout::Layout;
use crate::window::{Window, Geometry, WindowId};
use std::fmt;
use log::{debug};

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl fmt::Debug for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} - {:?} - {:?} - {:?}]", self.get_name(), &self.layout, &self.view, &self.windows)
    }
}

pub struct Workspace {
    name: String,
    layout: Box<dyn Layout>,
    is_changed: bool,
    pub view: Geometry,
    pub windows: Vec<Window>,
}

impl Workspace {
    pub fn new(name: String, windows: Vec<Window>, layout: Box<dyn Layout>, view: Geometry) -> Self {
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

    pub fn add_window(mut self, window: Window) -> Self {
        if self.windows.iter().find(|&w| w == &window).is_none() {
            self.windows.push(window);
            self.perform_layout()
        } else {
            self
        }
    }

    pub fn remove_window(mut self, window: WindowId) -> Self {
        let position = self.windows.iter().position(|w| w.get_id() == &window);
        if let Some(position) = position {
            self.windows.remove(position);
            self.perform_layout()
        } else {
            self
        }
    }

    pub fn change_layout<L: Layout + 'static>(mut self, layout: L) -> Self {
        self.layout = Box::new(layout);
        self.perform_layout()
    }

    fn perform_layout(self) -> Self {
        debug!("Updating layout {:?} for workspace {}", &self.layout, &self.name);
        let handled_windows = self.layout.handle_layout(&self.view, self.windows);
        Self {
            windows: handled_windows,
            is_changed: true,
            ..self
        }
    }
}
