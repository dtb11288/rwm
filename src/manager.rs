use crate::config::Config;
use crate::state::State;
use crate::displays::DisplayServer;
use std::collections::HashMap;
use crate::command::Command;

pub struct Manager<D: DisplayServer> {
    config: Config,
    display: D,
    handlers: HashMap<D::KeyCombo, Command>,
}

impl<D: DisplayServer> Manager<D> {
    pub fn new(config: Config) -> Self {
        let display = D::new(&config);
        let handlers = Command::new(&config);
        Manager { config, display, handlers }
    }

    fn update(&self, state: &State<D::Window>) {
        if state.quit {
            log::debug!("Close WM ...");
            self.display.quit()
        } else {
            state.workspaces.iter()
                .filter(|&w| w.is_changed())
                .for_each(|workspace| {
                    log::debug!("Update workspace {:?}", workspace);
                    workspace.iter()
                        .for_each(|window| {
                            self.display.configure_window(window);
                            self.display.set_visibility(&window, window.is_visible());
                        })
                });
        }
    }

    pub fn run(self) {
        log::info!("Start WM ...");
        let state = State::new(&self.config, self.display.get_root_view());
        self.display.clone().into_iter()
            .fold(state, move |state, event| {
                log::debug!("Received event {:?}", &event);
                let state = state.handle_event(event, &self.handlers);
                self.update(&state);
                state.reset()
            });
    }
}
