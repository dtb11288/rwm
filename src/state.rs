use crate::workspace::Workspace;
use crate::window::{WindowType, Window, Geometry};
use crate::config::Config;
use crate::displays::Event;
use crate::stack::Stack;
use crate::layouts::Layout;
use crate::keys::KeyCombo;
use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;
use crate::command::Command;
use crate::screen::Screen;

pub struct State<W> {
    pub quit: bool,
    pub workspaces: Stack<Workspace<W>>,
    pub screens: Stack<Screen<W>>,
}

impl<W: Debug + Clone + Eq> State<W> {
    pub fn new(config: &Config) -> Self {
        let layouts: Stack<Layout> = config.layouts.clone().into();
        let workspaces = config.workspaces.iter()
            .map(|name| Workspace::new(name.clone(), Stack::new(), layouts.clone()))
            .collect::<Vec<Workspace<W>>>()
            .into();

        Self { quit: false, workspaces, screens: Stack::new() }
    }

    pub fn reset(mut self) -> Self {
        self.workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| (is_current, workspace.reset()))
            .collect();
        self
    }

    pub fn handle_event<K: From<KeyCombo> + Hash + Eq + Debug>(self, event: Event<W, K>, handlers: &HashMap<K, Command>) -> Self {
        log::debug!("Handling event {:?}", event);
        match event {
            Event::WindowAdded(window, window_type) => {
                self.add_window(window, window_type)
            },
            Event::WindowRemoved(window) => {
                self.remove_window(window)
            },
            Event::KeyPressed(key) => {
                self.key_pressed(key, handlers)
            },
            Event::ScreenAdded(window, view) => {
                self.add_screen(window, view)
            }
            _ => self
        }
    }

    fn key_pressed<K: From<KeyCombo> + Hash + Eq + Debug>(self, key: K, handlers: &HashMap<K, Command>) -> Self {
        if let Some(command) = handlers.get(&key) {
            command.execute(self)
        } else {
            self
        }
    }

    fn add_screen(mut self, window: W, view: Geometry) -> Self {
        self.screens = self.screens.add(Screen::new(window, view));
        self.update_workspace_view()
    }

    fn update_workspace_view(mut self) -> Self {
        if let Some(screen) = self.screens.get_current() {
            self.workspaces = self.workspaces
                .update_current(|workspace| workspace.set_view(screen.get_view().clone()))
        }
        self
    }

    pub fn goto_workspace(mut self, position: usize) -> Self {
        self.workspaces = self.workspaces
            .update_current(|workspace| workspace.visible(false))
            .set_current(position)
            .update_current(|workspace| workspace.visible(true));
        self.update_workspace_view()
    }

    pub fn next_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_current(Workspace::next_window);
        self
    }

    pub fn previous_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_current(Workspace::previous_window);
        self
    }

    pub fn add_window(mut self, window: W, window_type: WindowType) -> Self {
        let window = Window::new(window, window_type).visible(true);
        self.workspaces = self.workspaces.update_current(move |workspace| workspace.add_window(window));
        self
    }

    pub fn quit(mut self) -> Self {
        self.quit = true;
        self
    }

    pub fn remove_window(mut self, window: W) -> Self {
        self.workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| {
                (is_current, workspace.remove_window(window.clone()))
            })
            .collect();
        self
    }
}
