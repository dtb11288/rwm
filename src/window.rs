#[derive(Debug)]
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

#[derive(Debug)]
pub struct Window<W> {
    id: W,
    window_type: WindowType,
    pub view: View,
}

#[derive(Debug, Default)]
pub struct View {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl View {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

impl<W> Window<W> {
    pub fn into(&self) -> &W {
        &self.id
    }

    pub fn new(id: W, window_type: WindowType) -> Self {
        Window { id, window_type, view: View::default() }
    }

    pub fn set_view(mut self, view: View) -> Self {
        self.view = view;
        self
    }
}
