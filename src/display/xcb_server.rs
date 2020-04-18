use crate::config::Config;
use crate::display::{DisplayServer, DisplayEvent};

pub type Root = u32;

pub struct XcbDisplay {
    config: Config,
    root: Root,
    connection: xcb::Connection,
}

impl From<xcb::GenericEvent> for DisplayEvent {
    fn from(event: xcb::GenericEvent) -> Self {
        match event.response_type() {
            xcb::KEY_PRESS => {
                let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                DisplayEvent::KeyPress("i".into())
            },
            _ => DisplayEvent::DoNothing,
        }
    }
}

impl DisplayServer for XcbDisplay {
    fn new(config: Config) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = connection.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        XcbDisplay {
            config,
            root,
            connection,
        }
    }

    fn run<F: Fn(DisplayEvent)>(&self, handler: F) {
        let events = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_BUTTON_PRESS |
                xcb::EVENT_MASK_BUTTON_RELEASE |
                xcb::EVENT_MASK_KEY_PRESS |
                xcb::EVENT_MASK_EXPOSURE |
                xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |
                xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
        )];

        let cookie = xcb::change_window_attributes(&self.connection, self.root, &events);

        if !cookie.request_check().is_ok() {
            panic!("There's another Window Manager Running!");
        }

        self.connection.flush();

        loop {
            match self.connection.wait_for_event() {
                Some(event) => handler(event.into()),
                None => {},
            }
        }
    }
}
