use log::{info, debug};
use crate::display::{DisplayServer, Event};
use crate::screen::Screen;
use crate::config::Config;
use crate::workspace::Workspace;
use crate::window::{Window, WindowType};
use crate::layouts::tall::Tall;

pub struct Manager<D: DisplayServer> {
    config: Config,
    display: D,
    screens: Vec<Screen>,
    current: String,
    workspaces: Vec<Workspace<D::Window>>,
}

impl<D: DisplayServer> Manager<D> {
    pub fn new(config: Config) -> Self {
        let display = D::new(config.clone());
        let workspaces = config.tags.iter().map(|w| {
            Workspace::new(w.clone(), vec![], Box::new(Tall {}))
        }).collect();
        Manager {
            config,
            display,
            current: "1".into(),
            screens: vec![],
            workspaces,
        }
    }

    fn handle_event(&self, event: Event<D::Window>) {
        match event {
            Event::WindowAdded(window, window_type) => {
                self.add_window(window, window_type)
            },
            Event::KeyPressed(_key) => {
                // std::process::exit(1);
            },
            _ => {}
        }
    }

    fn add_window(&self, window: D::Window, window_type: WindowType) {
        let window = Window::new(window, window_type);
        let mut current = (&self.workspaces).iter().find(|w| w.get_name() == &self.current);
        // if let Some(workspace) = current.as_mut() {
        //     workspace.add_window(window);
        //     workspace.windows.iter().for_each(|window| {
        //         (&self.display).configure_window(window);
        //     });
        // }
    }

    pub fn run(self) {
        info!("Start WM ...");
        self.display.run(|event| {
            let event = self.display.match_event(event);
            debug!("Received event {:?}", &event);
            self.handle_event(event)
        })
    }
}
