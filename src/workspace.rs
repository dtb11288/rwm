use crate::layouts::Layout;
use crate::window::{Window, Geometry};
use std::ops::Deref;
use crate::stack::Stack;
use std::fmt::{self, Debug};

impl<W: Debug + Eq + Clone> PartialEq for Workspace<W> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl<W: Debug + Eq + Clone> Debug for Workspace<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} - {:?} - {:?} - {:?}]", self.get_name(), self.layouts.get_current().unwrap(), &self.view, &self.windows)
    }
}

#[derive(Clone)]
pub struct Workspace<W> {
    name: String,
    is_changed: bool,
    view: Geometry,
    windows: Stack<Window<W>>,
    layouts: Stack<Layout>,
}

impl<W> Deref for Workspace<W> {
    type Target = Stack<Window<W>>;

    fn deref(&self) -> &Self::Target {
        &self.windows
    }
}

impl<W: Debug + Eq + Clone> Workspace<W> {
    pub fn new(name: String, windows: Stack<Window<W>>, layouts: Stack<Layout>, view: Geometry) -> Self {
        let workspace = Self { name, windows, layouts, is_changed: false, view };
        workspace.perform_layout()
    }

    pub fn visible(mut self, visible: bool) -> Self {
        if visible {
            self.perform_layout()
        } else {
            self.windows = self.windows.into_iter()
                .map(|(is_current, window)| (is_current, window.visible(false)))
                .collect();
            self.need_update()
        }
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

    pub fn next_window(mut self) -> Self {
        log::debug!("Focus next window");
        self.windows = self.windows.next();
        self.perform_layout()
    }

    pub fn previous_window(mut self) -> Self {
        log::debug!("Focus previous window");
        self.windows = self.windows.previous();
        self.perform_layout()
    }

    pub fn add_window(mut self, window: Window<W>) -> Self {
        log::debug!("Adding window id {:?} to workspace {}", &window.deref(), self.get_name());
        self.windows = self.windows.add(window);
        self.perform_layout()
    }

    pub fn remove_window(mut self, window: W) -> Self {
        log::debug!("Removing window id {:?} from workspace {}", &window, self.get_name());
        let old_len = self.windows.len();
        self.windows = self.windows.remove_by(|w| w.deref() == &window);
        if old_len != self.windows.len() {
            self.perform_layout()
        } else {
            self
        }
    }

    fn perform_layout(mut self) -> Self {
        if self.windows.is_empty() {
            return self;
        }
        let layout = self.layouts.get_current().unwrap();
        log::debug!("Updating layout for workspace {} using {:?}", &self.name, &layout);
        let handled_windows = layout.handle_layout(&self.view, self.windows);
        self.windows = handled_windows;
        self.need_update()
    }
}
