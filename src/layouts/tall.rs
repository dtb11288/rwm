use crate::layout::Layout;
use crate::window::{Window, View};

pub struct Tall {}

impl<W: Clone + PartialEq> Layout<W> for Tall {
    fn get_name(&self) -> &str {
        "tall"
    }

    fn handle_layout(&self, workspace_view: &View, windows: Vec<Window<W>>) -> Vec<Window<W>> {
        let window_count = windows.len();
        if window_count == 0 { return windows; };
        windows.into_iter().enumerate()
            .map(|(pos, window)| {
                let width = if pos == 0 && window_count == 1 { workspace_view.width } else { workspace_view.width / 2 };
                let height = if pos == 0 { workspace_view.height } else { workspace_view.height / (window_count as u32 - 1) };
                let x = if pos == 0 { workspace_view.x } else { width };
                let y = if pos == 0 { workspace_view.y } else { height * (pos as u32 - 1) };
                window.set_view(View { x, y, width, height })
            })
            .collect()
    }
}
