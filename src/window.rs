use core::fmt::{self, Debug};
use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl<W> Deref for Window<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[derive(Clone)]
pub struct Window<W> {
    id: W,
    window_type: WindowType,
    view: Option<Geometry>,
    visible: bool,
}

impl<W: Eq> PartialEq for Window<W> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.position, self.size)
    }
}

impl<W: Debug> fmt::Debug for Window<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.view.is_none() {
            write!(f, "({:?} - {:?} - {})", &self.id, &self.window_type, "Unknown")
        } else {
            write!(f, "({:?} - {:?} - {:?})", &self.id, &self.window_type, self.view.as_ref().unwrap())
        }
    }
}

#[derive(Default, Clone)]
pub struct Geometry {
    pub size: Size,
    pub position: Position,
}

#[derive(Default, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Geometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            size: Size { width, height },
            position: Position { x, y },
        }
    }
}

impl<W> Window<W> {
    pub fn new(id: W, window_type: WindowType) -> Self {
        Window { id, window_type, view: None, visible: false }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn set_view(mut self, view: Geometry) -> Self {
        self.view.replace(view);
        self
    }

    pub fn get_view(&self) -> &Geometry {
        self.view.as_ref().unwrap()
    }
}
