use crate::config::Config;
use crate::display::{DisplayServer, DisplayEvent};

pub struct Manager<D: DisplayServer> {
    config: Config,
    display: D,
}

impl<D: DisplayServer> Manager<D> {
    pub fn new(config: Config) -> Self {
        let display = D::new(config.clone());
        Manager {
            config,
            display,
        }
    }

    fn handle_event(&self, event: DisplayEvent) {
        dbg!(event);
    }

    pub fn run(self) {
        self.display.run(|event| self.handle_event(event))
    }
}
