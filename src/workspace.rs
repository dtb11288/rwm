use crate::layouts::LayoutType;
use crate::window::{Window, Geometry, WindowId};
use std::fmt;
use std::ops::Deref;
use crate::stack::Stack;

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl fmt::Debug for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} - {:?} - {:?} - {:?}]", self.get_name(), self.layouts.get_focus().unwrap(), &self.view, &self.windows)
    }
}

#[derive(Clone)]
pub struct Workspace {
    name: String,
    is_changed: bool,
    view: Geometry,
    windows: Stack<Window>,
    layouts: Stack<LayoutType>,
}

impl Deref for Workspace {
    type Target = Stack<Window>;

    fn deref(&self) -> &Self::Target {
        &self.windows
    }
}

impl Workspace {
    pub fn new(name: String, windows: Stack<Window>, layouts: Stack<LayoutType>, view: Geometry) -> Self {
        let workspace = Self { name, windows, layouts, is_changed: false, view };
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
        log::debug!("Adding window id {:?} to workspace {}", &window.deref(), self.get_name());
        self.windows = self.windows.add(window);
        self.perform_layout()
    }

    pub fn remove_window(mut self, window: WindowId) -> Self {
        log::debug!("Removing window id {:?} from workspace", &window);
        let old_len = self.windows.len();
        self.windows = self.windows.remove_by(|w| w.deref() == &window);
        if old_len != self.windows.len() {
            self.perform_layout()
        } else {
            self
        }
    }

    fn perform_layout(self) -> Self {
        if self.windows.is_empty() {
            return self
        }
        log::debug!("Updating layout for workspace {} using {:?}", &self.name, &self.layouts.get_focus().unwrap());
        let handled_windows = self.layouts.get_focus().unwrap().handle_layout(&self.view, self.windows);
        Self {
            windows: handled_windows,
            ..self
        }.need_update()
    }
}
