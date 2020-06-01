use crate::state::State;
use std::fmt::Debug;
use crate::keys::{KeyCombo, Key, ModKey};
use std::hash::Hash;
use crate::config::Config;
use std::collections::HashMap;

pub enum Command {
    Spawn(String),
    NextWindow,
    PreviousWindow,
    GoToWorkspace(usize),
    Quit,
}

impl Command {
    pub fn new<K: From<KeyCombo> + Hash + Eq + Debug>(config: &Config) -> HashMap<K, Self> {
        let mut commands = HashMap::new();
        commands.insert(
            KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('p') }.into(),
            Command::Spawn("dmenu_run".to_string())
        );
        commands.insert(
            KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('j') }.into(),
            Command::NextWindow
        );
        commands.insert(
            KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key('k') }.into(),
            Command::PreviousWindow
        );
        commands.insert(
            KeyCombo { mod_keys: vec![config.mod_key.clone(), ModKey::Shift], key: Key('q') }.into(),
            Command::Quit
        );
        commands.insert(
            KeyCombo { mod_keys: vec![config.mod_key.clone(), ModKey::Shift], key: Key('u') }.into(),
            Command::Spawn("urxvt".to_string())
        );
        for pos in b'1'..=b'9' {
            let index = usize::from(pos - 49);
            let pos = char::from(pos);
            commands.insert(
                KeyCombo { mod_keys: vec![config.mod_key.clone()], key: Key(pos as char) }.into(),
                Command::GoToWorkspace(index)
            );
        }
        commands
    }
    pub fn execute<W: Debug + Clone + Eq>(&self, state: State<W>) -> State<W> {
        match self {
            Command::Spawn(command) => {
                std::process::Command::new(command.as_str()).spawn().ok();
                state
            },
            Command::NextWindow => state.next_window(),
            Command::PreviousWindow => state.previous_window(),
            Command::GoToWorkspace(index) => state.goto_workspace(*index),
            Command::Quit => state.quit()
        }
    }
}
