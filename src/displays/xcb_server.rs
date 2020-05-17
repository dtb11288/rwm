use crate::displays::{DisplayServer, Event};
use crate::config::Config;
use crate::window::{Window, WindowType, Geometry};
use std::rc::Rc;
use xcb_util::ewmh;

#[derive(Clone)]
pub struct XcbDisplayServer {
    root: xcb::Window,
    connection: Rc<ewmh::Connection>,
}

impl Iterator for XcbDisplayServer {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.connection.flush();
        self.connection.wait_for_event()
            .map(|event| self.match_event(event))
    }
}

impl DisplayServer for XcbDisplayServer {
    fn get_root_view(&self) -> Geometry {
        let reply = xcb::get_geometry(&self.connection, self.root)
            .get_reply()
            .unwrap();
        Geometry::new(0, 0, u32::from(reply.width()), u32::from(reply.height()))
    }

    fn configure_window(&self, window: &Window) {
        let view = window.get_view();
        let values = [
            (xcb::CONFIG_WINDOW_X as u16, view.position.x as u32),
            (xcb::CONFIG_WINDOW_Y as u16, view.position.y as u32),
            (xcb::CONFIG_WINDOW_WIDTH as u16, view.size.width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, view.size.height),
        ];
        let window_id = window.parse::<xcb::Window>().unwrap();
        xcb::configure_window(&self.connection, window_id, &values);
        xcb::map_window(&self.connection, window_id);
    }

    fn close_window(&self, _window: &Window) {
        unimplemented!()
    }
}

impl XcbDisplayServer {
    pub fn new(_config: &Config) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();
        let connection = ewmh::Connection::connect(connection).map_err(|e| e.0).unwrap();
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
            root,
            connection: Rc::new(connection),
        }
    }

    fn match_event(&self, event: xcb::GenericEvent) -> Event {
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
                Event::WindowAdded(map_request.window().to_string(), WindowType::Normal)
            },
            xcb::UNMAP_NOTIFY => {
                let unmap_notify: &xcb::UnmapNotifyEvent = unsafe { xcb::cast_event(&event) };
                if unmap_notify.event() == self.root {
                    Event::WindowRemoved(unmap_notify.event().to_string())
                } else {
                    Event::Ignored
                }
            },
            xcb::DESTROY_NOTIFY => {
                let destroy_event: &xcb::DestroyNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowRemoved(destroy_event.window().to_string())
            },
            xcb::ENTER_NOTIFY => {
                let enter_event: &xcb::EnterNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowFocused(enter_event.event().to_string())
            },
            _ => Event::Ignored,
        }
    }
}
