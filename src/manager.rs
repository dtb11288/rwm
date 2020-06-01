use crate::config::Config;
use crate::state::State;
use crate::displays::DisplayServer;

pub struct Manager {
    config: Config,
    display: Box<dyn DisplayServer>,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        let display = config.display.init(&config);
        Manager { config, display }
    }

    fn update(&self, state: &State) {
        if state.quit {
            log::debug!("Quit");
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
                let state = state.handle_event(event);
                self.update(&state);
                state.reset()
            });
    }
}
