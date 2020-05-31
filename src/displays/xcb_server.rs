use crate::displays::{DisplayServer, Event};
use crate::config::Config;
use crate::window::{Window, WindowType, Geometry, WindowId};
use std::rc::Rc;
use xcb_util::ewmh;
use xcb_util::keysyms::KeySymbols;
use crate::keys::xcb_keys::XcbKeyCombo;
use std::cell::RefCell;

#[derive(Clone)]
pub struct XcbDisplayServer {
    stopped: Rc<RefCell<bool>>,
    root: xcb::Window,
    connection: Rc<ewmh::Connection>,
}

impl Iterator for XcbDisplayServer {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.stopped.borrow_mut() { return None }
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
        let events = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_BUTTON_PRESS |
                xcb::EVENT_MASK_BUTTON_RELEASE |
                xcb::EVENT_MASK_KEY_PRESS
        )];
        xcb::change_window_attributes(&self.connection, window_id, &events);
    }

    fn set_visibility(&self, window: &WindowId, show: bool) {
        let window_id = window.parse::<xcb::Window>().unwrap();
        if show {
            xcb::map_window(&self.connection, window_id);
        } else {
            xcb::unmap_window(&self.connection, window_id);
        }
    }

    fn quit(&self) {
        *self.stopped.borrow_mut() = true;
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
            stopped: Rc::new(RefCell::new(false)),
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
                let key_symbols = KeySymbols::new(&self.connection);
                let keysym = key_symbols.press_lookup_keysym(key_press, 0);
                let mod_mask = u32::from(key_press.state());
                let key_combo = XcbKeyCombo { mod_mask, key: keysym };
                Event::KeyPressed(key_combo)
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
