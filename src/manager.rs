use log::{info, debug};
use crate::display::DisplayServer;
use crate::config::Config;
use crate::state::State;

pub struct Manager<D> {
    config: Config,
    display: D,
}

impl<D: DisplayServer + Clone> Manager<D> {
    pub fn new(config: Config) -> Self {
        info!("Start WM ...");
        let display = D::new(&config);
        Manager {
            config,
            display,
        }
    }

    pub fn update(&self, state: &State<D::Window>) {
        state.workspaces.iter()
            .filter(|w| w.is_changed())
            .for_each(|w| {
                debug!("Update workspace {} with {} windows", w.get_name(), w.windows.len());
                w.windows.iter()
                    .for_each(|w| {
                        dbg!(&w);
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
