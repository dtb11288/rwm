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

#[derive(Debug, Clone)]
pub struct Window<W: Clone + PartialEq> {
    id: W,
    window_type: WindowType,
    pub view: View,
}

impl<W: PartialEq + Clone> PartialEq for Window<W> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

#[derive(Debug, Default, Clone)]
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

impl<W: Clone + PartialEq> Window<W> {
    pub fn get_id(&self) -> &W {
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
