use crate::window::{Window, Geometry};
use crate::stack::Stack;

pub fn handle_layout<W>(view: &Geometry, windows: Stack<Window<W>>) -> Stack<Window<W>> {
    let window_count = windows.len();
    if window_count == 0 { return windows; };
    windows.into_iter().enumerate()
        .map(|(pos, (is_current, window))| {
            let width = if pos == 0 && window_count == 1 { view.size.width } else { view.size.width / 2 };
            let height = if pos == 0 { view.size.height } else { view.size.height / (window_count as u32 - 1) };
            let x = if pos == 0 { view.position.x } else { width as i32 } + view.position.x;
            let y = if pos == 0 { view.position.y } else { height as i32 * (pos as i32 - 1) } + view.position.y;
            (is_current, window.set_view(Geometry::new(x, y, width, height)).visible(true))
        })
        .collect()
}
