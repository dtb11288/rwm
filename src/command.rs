use crate::state::State;

pub type Command = Box<dyn Fn(&State) -> State>;

pub fn spawn(command: String) -> Command {
    Box::new(move |state| {
        std::process::Command::new(command.as_str()).spawn().ok();
        state.clone()
    })
}

pub fn next_window() -> Command {
    Box::new(move |state| {
        state.clone().next_window()
    })
}

pub fn previous_window() -> Command {
    Box::new(move |state| {
        state.clone().previous_window()
    })
}

pub fn goto_workspace(position: usize) -> Command {
    Box::new(move |state| {
        state.clone().goto_workspace(position)
    })
}

pub fn quit() -> Command {
    Box::new(move |state| {
        state.clone().quit()
    })
}
