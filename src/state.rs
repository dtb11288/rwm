use crate::workspace::Workspace;
use crate::display::Event;
use crate::window::{WindowType, Window, Geometry, WindowId};
use crate::config::Config;
use crate::layouts::LayoutType;
use log::debug;

pub struct State {
    current: String,
    pub view: Geometry,
    pub workspaces: Vec<Workspace>,
    pub layouts: Vec<LayoutType>,
}

impl State {
    pub fn new(config: &Config, view: Geometry) -> Self {
        let layouts = config.layouts.clone();
        let first_layout = config.layouts.first().unwrap();
        let workspaces = config.workspaces.iter()
            .map(|w| Workspace::new(w.clone(), vec![], first_layout.clone().into(), view.clone()))
            .collect::<Vec<Workspace>>();
        let first_workspace = workspaces.first().unwrap();
        Self {
            view,
            current: first_workspace.get_name().to_string(),
            workspaces,
            layouts,
        }
    }

    pub fn reset(self) -> Self {
        let workspaces = self.workspaces.into_iter()
            .map(Workspace::reset)
            .collect();
        Self {
            workspaces,
            ..self
        }
    }

    pub fn handle_event(self, event: Event) -> Self {
        debug!("Handling event {:?}", event);
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
        debug!("Adding window id {:?}", &window);
        let window = Window::new(window, window_type);
        let current = &self.current;
        let workspaces = self.workspaces.into_iter()
            .map(move |workspace| {
                if workspace.get_name() == current {
                    workspace.add_window(window.clone())
                } else {
                    workspace
                }
            })
            .collect::<Vec<Workspace>>();
        Self {
            workspaces,
            ..self
        }
    }

    fn remove_window(self, window: WindowId) -> Self {
        debug!("Removing window id {:?}", &window);
        let workspaces = self.workspaces.into_iter()
            .map(|workspace| {
                workspace.remove_window(window.clone())
            })
            .collect();
        Self {
            workspaces,
            ..self
        }
    }
}
