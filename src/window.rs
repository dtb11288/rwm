use core::fmt::{self, Debug};

#[derive(Debug, Clone)]
pub enum WindowType {
    Desktop,
    Dock,
    Toolbar,
    Menu,
    Utility,
    Splash,
    Dialog,
    DropdownMenu,
    PopupMenu,
    Tooltip,
    Notification,
    Combo,
    Dnd,
    Normal,
}

pub type WindowId = String;

#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    pub window_type: WindowType,
    pub view: Geometry,
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{} {}:{}", self.width, self.height, self.x, self.y)
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} - {:?} - {:?})", &self.id, &self.window_type, &self.view)
    }
}

#[derive(Default, Clone)]
pub struct Geometry {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn get_id(&self) -> &WindowId {
        &self.id
    }

    pub fn new(id: WindowId, window_type: WindowType) -> Self {
        Window { id, window_type, view: Geometry::default() }
    }

    pub fn set_view(mut self, view: Geometry) -> Self {
        self.view = view;
        self
    }
}
