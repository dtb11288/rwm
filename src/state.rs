use crate::workspace::Workspace;
use crate::window::{WindowType, Window, Geometry, WindowId};
use crate::config::Config;
use crate::displays::Event;
use crate::stack::Stack;
use crate::layouts::LayoutType;

pub struct State {
    pub view: Geometry,
    pub workspaces: Stack<Workspace>,
}

impl State {
    pub fn new(config: &Config, view: Geometry) -> Self {
        let layouts: Stack<LayoutType> = config.layouts.clone().into();
        let workspaces = config.workspaces.iter()
            .map(|name| Workspace::new(name.clone(), Stack::new(), layouts.clone(), view.clone()))
            .collect::<Vec<Workspace>>()
            .into();
        Self { view, workspaces }
    }

    pub fn reset(self) -> Self {
        let workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| (is_current, workspace.reset()))
            .collect();
        Self {
            workspaces,
            ..self
        }
    }

    pub fn handle_event(self, event: Event) -> Self {
        log::debug!("Handling event {:?}", event);
        match event {
            Event::WindowAdded(window, window_type) => {
                self.add_window(window, window_type)
            },
            Event::WindowRemoved(window) => {
                self.remove_window(window)
            },
            Event::KeyPressed(_key) => {
                self
            },
            _ => self
        }
    }

    fn add_window(self, window: WindowId, window_type: WindowType) -> Self {
        let window = Window::new(window, window_type);
        let workspaces = self.workspaces.into_iter()
            .map(move |(is_current, workspace)| {
                let workspace = if is_current {
                    workspace.add_window(window.clone())
                } else {
                    workspace
                };
                (is_current, workspace)
            })
            .collect::<Stack<Workspace>>();
        Self {
            workspaces,
            ..self
        }
    }

    fn remove_window(self, window: WindowId) -> Self {
        let workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| {
                (is_current, workspace.remove_window(window.clone()))
            })
            .collect();
        Self {
            workspaces,
            ..self
        }
    }
}
