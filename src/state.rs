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
            Event::KeyPressed(key) => {
                self.key_pressed(key)
            },
            _ => self
        }
    }

    fn key_pressed(self, key: String) -> Self {
        match key.as_str() {
            "8 49" => {
                std::process::Command::new("urxvt").spawn().ok();
                self
            },
            "8 50" => {
                self.next_workspace()
            },
            "8 51" => {
                self.previous_workspace()
            },
            _ => self
        }
    }

    fn next_workspace(self) -> Self {
        let workspaces = self.workspaces
            .update_focus(|workspace| workspace.invisible())
            .next()
            .update_focus(|workspace| workspace.visible());
        Self {
            workspaces,
            ..self
        }
    }

    fn previous_workspace(self) -> Self {
        let workspaces = self.workspaces
            .update_focus(|workspace| workspace.invisible())
            .previous()
            .update_focus(|workspace| workspace.visible());
        Self {
            workspaces,
            ..self
        }
    }

    fn next_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_focus(Workspace::next_window);
        self
    }

    fn previous_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_focus(Workspace::previous_window);
        self
    }

    fn add_window(self, window: WindowId, window_type: WindowType) -> Self {
        let window = Window::new(window, window_type).visible(true);
        let workspaces = self.workspaces.update_focus(move |workspace| workspace.add_window(window));
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
