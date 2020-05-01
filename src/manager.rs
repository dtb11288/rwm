use log::{info, debug};
use crate::display::DisplayServer;
use crate::config::Config;
use crate::state::State;
use crate::displays::xcb_server::XcbDisplayServer;
use crate::displays::DisplayType;

pub struct Manager {
    config: Config,
    display: Box<dyn DisplayServer>,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        let display = match &config.display {
            DisplayType::Xcb => XcbDisplayServer::new(&config),
        };
        Manager {
            config,
            display: Box::new(display),
        }
    }

    pub fn update(&self, state: &State) {
        state.workspaces.iter()
            .filter(|&w| w.is_changed())
            .for_each(|w| {
                debug!("Update workspace {:?}", w);
                w.windows.iter()
                    .for_each(|w| {
                        self.display.configure_window(w)
                    })
            });
    }

    pub fn run(&self) {
        info!("Start WM ...");
        let state = State::new(&self.config, self.display.get_root_view());
        self.display.clone().into_iter()
            .fold(state, move |state, event| {
                debug!("Received event {:?}", &event);
                let state = state.handle_event(event);
                self.update(&state);
                state.reset()
            });
    }
}
