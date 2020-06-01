use crate::state::State;

pub type Command = Box<dyn Fn(State) -> State>;

pub fn spawn(command: String) -> Command {
    Box::new(move |state| {
        std::process::Command::new(command.as_str()).spawn().ok();
        state
    })
}

pub fn next_window() -> Command {
    Box::new(move |state| {
        state.next_window()
    })
}

pub fn previous_window() -> Command {
    Box::new(move |state| {
        state.previous_window()
    })
}

pub fn goto_workspace(position: usize) -> Command {
    Box::new(move |state| {
        state.goto_workspace(position)
    })
}

pub fn quit() -> Command {
    Box::new(move |state| {
        state.quit()
    })
}
