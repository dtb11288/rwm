use crate::workspace::Workspace;
use crate::window::{WindowType, Window, Geometry, WindowId};
use crate::config::Config;
use crate::displays::Event;
use crate::stack::Stack;
use crate::layouts::LayoutType;
use std::collections::HashMap;
use crate::command::Command;
use crate::keys::{KeyCombo, Key, KeyConvert, ModKey};
use crate::keys::xcb_keys::{XcbKeyCombo, XcbKeyConvert};
use std::rc::Rc;
use std::cell::RefCell;

pub struct State {
    pub quit: bool,
    pub view: Geometry,
    pub workspaces: Stack<Workspace>,
    commands: Rc<RefCell<HashMap<XcbKeyCombo, Command>>>,
}

impl State {
    pub fn new(config: &Config, view: Geometry) -> Self {
        let layouts: Stack<LayoutType> = config.layouts.clone().into();
        let workspaces = config.workspaces.iter()
            .map(|name| Workspace::new(name.clone(), Stack::new(), layouts.clone(), view.clone()))
            .collect::<Vec<Workspace>>()
            .into();

        let mut commands = HashMap::new();
        commands.insert(
            XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('p') }),
            crate::command::spawn("dmenu_run".to_string())
        );
        commands.insert(
            XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('j') }),
            crate::command::next_window()
        );
        commands.insert(
            XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('k') }),
            crate::command::previous_window()
        );
        commands.insert(
            XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone(), ModKey::Shift], key: Key('q') }),
            crate::command::quit()
        );
        commands.insert(
            XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone(), ModKey::Shift], key: Key('u') }),
            crate::command::spawn("urxvt".to_string())
        );
        for pos in b'1'..=b'9' {
            let index = usize::from(pos - 49);
            let pos = char::from(pos);
            commands.insert(
                XcbKeyConvert.convert(KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key(pos as char) }),
                crate::command::goto_workspace(index as usize)
            );
        }

        Self { quit: false, view, workspaces, commands: Rc::new(RefCell::new(commands)) }
    }

    pub fn reset(mut self) -> Self {
        self.workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| (is_current, workspace.reset()))
            .collect();
        self
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

    fn key_pressed(self, key: XcbKeyCombo) -> Self {
        let command = self.commands.borrow_mut().remove(&key);
        if let Some(command) = command {
            let state = command(self);
            state.commands.borrow_mut().insert(key, command);
            state
        } else {
            self
        }
    }

    pub fn goto_workspace(mut self, position: usize) -> Self {
        self.workspaces = self.workspaces
            .update_current(|workspace| workspace.visible(false))
            .set_current(position)
            .update_current(|workspace| workspace.visible(true));
        self
    }

    fn next_workspace(mut self) -> Self {
        self.workspaces = self.workspaces
            .update_current(|workspace| workspace.visible(false))
            .next()
            .update_current(|workspace| workspace.visible(true));
        self
    }

    fn previous_workspace(mut self) -> Self {
        self.workspaces = self.workspaces
            .update_current(|workspace| workspace.visible(false))
            .previous()
            .update_current(|workspace| workspace.visible(true));
        self
    }

    pub fn next_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_current(Workspace::next_window);
        self
    }

    pub fn previous_window(mut self) -> Self {
        self.workspaces = self.workspaces.update_current(Workspace::previous_window);
        self
    }

    pub fn add_window(mut self, window: WindowId, window_type: WindowType) -> Self {
        let window = Window::new(window, window_type).visible(true);
        self.workspaces = self.workspaces.update_current(move |workspace| workspace.add_window(window));
        self
    }

    pub fn quit(mut self) -> Self {
        self.quit = true;
        self
    }

    pub fn remove_window(mut self, window: WindowId) -> Self {
        self.workspaces = self.workspaces.into_iter()
            .map(|(is_current, workspace)| {
                (is_current, workspace.remove_window(window.clone()))
            })
            .collect();
        self
    }
}
