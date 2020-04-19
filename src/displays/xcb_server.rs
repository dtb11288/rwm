use crate::display::{DisplayServer, Event};
use crate::config::Config;
use crate::window::{Window, WindowType};

pub struct XcbDisplayServer {
    config: Config,
    root: xcb::Window,
    connection: xcb::Connection,
}

impl DisplayServer for XcbDisplayServer {
    type Event = xcb::GenericEvent;
    type Window = xcb::Window;

    fn new(config: Config) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = connection.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        let events = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_BUTTON_PRESS |
                xcb::EVENT_MASK_BUTTON_RELEASE |
                xcb::EVENT_MASK_KEY_PRESS |
                xcb::EVENT_MASK_EXPOSURE |
                xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |
                xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
        )];

        let cookie = xcb::change_window_attributes(&connection, root, &events);

        if !cookie.request_check().is_ok() {
            panic!("There's another Window Manager Running!");
        }

        XcbDisplayServer {
            config,
            root,
            connection,
        }
    }

    fn run<F>(&self, handler: F) where F: Fn(Self::Event) {
        loop {
            self.connection.flush();
            match self.connection.wait_for_event() {
                Some(event) => handler(event),
                None => {},
            }
        }
    }

    fn match_event(&self, event: Self::Event) -> Event<Self::Window> {
        match event.response_type() {
            xcb::CONFIGURE_REQUEST => {
                Event::Ignored
            },
            xcb::KEY_PRESS => {
                let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                dbg!(key_press.detail());
                Event::KeyPressed("q".into())
            },
            xcb::MAP_REQUEST => {
                let map_request: &xcb::MapRequestEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowAdded(map_request.window(), WindowType::Normal)
            },
            xcb::UNMAP_NOTIFY => {
                let unmap_notify: &xcb::UnmapNotifyEvent = unsafe { xcb::cast_event(&event) };
                if unmap_notify.event() == self.root {
                    Event::WindowRemoved(unmap_notify.event())
                } else {
                    Event::Ignored
                }
            },
            xcb::DESTROY_NOTIFY => {
                let destroy_event: &xcb::DestroyNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowRemoved(destroy_event.window())
            },
            xcb::ENTER_NOTIFY => {
                let enter_event: &xcb::EnterNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowFocused(enter_event.event())
            },
            _ => Event::Ignored,
        }
    }

    fn configure_window(&self, window: &Window<Self::Window>) {
        let view = &window.view;
        let values = [
            (xcb::CONFIG_WINDOW_X as u16, view.x),
            (xcb::CONFIG_WINDOW_Y as u16, view.y),
            (xcb::CONFIG_WINDOW_WIDTH as u16, view.width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, view.height),
        ];
        xcb::configure_window(&self.connection, *window.into(), &values);
    }

    fn close_window(&self, _window: &Window<Self::Window>) {
        unimplemented!()
    }
}