use crate::workspace::Workspace;
use crate::display::Event;
use crate::window::{WindowType, Window, View};
use crate::config::Config;
use crate::layouts::tall::Tall;
use std::fmt::Debug;
use log::{debug};

pub struct State<W: Clone + PartialEq + Debug> {
    current: String,
    pub view: View,
    pub workspaces: Vec<Workspace<W>>,
}

impl<W: Debug + Clone + PartialEq + Debug> State<W> {
    pub fn new(config: &Config, view: View) -> Self {
        let workspaces = config.tags.iter().map(|w| {
            Workspace::new(w.clone(), vec![], Box::new(Tall {}), view.clone())
        }).collect();
        Self {
            view,
            current: "1".into(),
            workspaces,
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

    pub fn handle_event(self, event: Event<W>) -> Self {
        debug!("Call handle_event for {:?}", event);
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

    fn add_window(self, window: W, window_type: WindowType) -> Self {
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
            .collect::<Vec<Workspace<W>>>();
        Self {
            workspaces,
            ..self
        }
    }

    fn remove_window(self, window: W) -> Self {
        debug!("Adding window id {:?}", &window);
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
