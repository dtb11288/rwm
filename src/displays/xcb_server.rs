use crate::displays::{DisplayServer, Event};
use crate::config::Config;
use crate::window::{WindowType, Geometry, Window};
use std::rc::Rc;
use xcb_util::ewmh;
use xcb_util::keysyms::KeySymbols;
use crate::keys::xcb_keys::XcbKeyCombo;
use std::cell::RefCell;
use std::ops::Deref;
use futures::Stream;
use futures::task::{Context, Poll};
use std::pin::Pin;

#[derive(Clone)]
pub struct XcbDisplayServer {
    connection: Rc<ewmh::Connection>,
    events: Rc<RefCell<Vec<Event<xcb::Window, XcbKeyCombo>>>>,
}

impl Stream for XcbDisplayServer {
    type Item = Event<xcb::Window, XcbKeyCombo>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.connection.flush();
        let mut events = self.events.borrow_mut();
        if events.len() == 0 {
            match self.connection.wait_for_event() {
                Some(event) => Poll::Ready(Some(self.match_event(event))),
                None => Poll::Pending,
            }
        } else {
            let event = events.remove(0);
            if event == Event::DisplayEnded { return Poll::Ready(None) }
            if let Some(event) = self.connection.poll_for_event() {
                events.push(self.match_event(event));
            };
            Poll::Ready(Some(event))
        }
    }
}

impl DisplayServer for XcbDisplayServer {
    type Window = xcb::Window;
    type KeyCombo = XcbKeyCombo;

    fn new(_config: &Config) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();
        let connection = ewmh::Connection::connect(connection).map_err(|e| e.0).unwrap();
        let setup = connection.get_setup();

        let events = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_BUTTON_PRESS |
                xcb::EVENT_MASK_BUTTON_RELEASE |
                xcb::EVENT_MASK_KEY_PRESS |
                xcb::EVENT_MASK_EXPOSURE |
                xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |
                xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY,
        )];

        let screens = setup.roots()
            .map(|screen| screen.root())
            .map(|screen| {
                let cookie = xcb::change_window_attributes(&connection, screen, &events);
                if !cookie.request_check().is_ok() {
                    panic!("There's another Window Manager Running!");
                }
                screen
            })
            .map(|screen| Event::ScreenAdded(screen, Self::get_screen_view(&connection, screen)))
            .collect();

        XcbDisplayServer {
            connection: Rc::new(connection),
            events: Rc::new(RefCell::new(screens)),
        }
    }

    fn configure_window(&self, window: &Window<xcb::Window>) {
        let view = window.get_view();
        let values = [
            (xcb::CONFIG_WINDOW_X as u16, view.position.x as u32),
            (xcb::CONFIG_WINDOW_Y as u16, view.position.y as u32),
            (xcb::CONFIG_WINDOW_WIDTH as u16, view.size.width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, view.size.height),
        ];
        let window_id = window.deref();
        xcb::configure_window(&self.connection, *window_id, &values);
        let events = [(
            xcb::CW_EVENT_MASK,
            xcb::EVENT_MASK_BUTTON_PRESS |
                xcb::EVENT_MASK_BUTTON_RELEASE |
                xcb::EVENT_MASK_KEY_PRESS
        )];
        xcb::change_window_attributes(&self.connection, *window_id, &events);
    }

    fn set_visibility(&self, window: &xcb::Window, show: bool) {
        if show {
            xcb::map_window(&self.connection, *window);
        } else {
            xcb::unmap_window(&self.connection, *window);
        }
    }

    fn quit(&self) {
        self.events.borrow_mut().push(Event::DisplayEnded)
    }
}

impl XcbDisplayServer {
    fn get_screen_view(connection: &xcb::Connection, screen: u32) -> Geometry {
        let reply = xcb::get_geometry(connection, screen)
            .get_reply()
            .unwrap();
        Geometry::new(0, 0, u32::from(reply.width()), u32::from(reply.height()))
    }

    fn match_event(&self, event: xcb::GenericEvent) -> Event<xcb::Window, XcbKeyCombo> {
        match event.response_type() {
            xcb::CONFIGURE_REQUEST => {
                Event::Ignored
            }
            xcb::KEY_PRESS => {
                let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                let key_symbols = KeySymbols::new(&self.connection);
                let keysym = key_symbols.press_lookup_keysym(key_press, 0);
                let mod_mask = u32::from(key_press.state());
                let key_combo = XcbKeyCombo { mod_mask, key: keysym };
                Event::KeyPressed(key_combo)
            }
            xcb::MAP_REQUEST => {
                let map_request: &xcb::MapRequestEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowAdded(map_request.window(), WindowType::Normal)
            }
            xcb::UNMAP_NOTIFY => {
                let unmap_notify: &xcb::UnmapNotifyEvent = unsafe { xcb::cast_event(&event) };
                // if unmap_notify.event() == self.root {
                    Event::WindowRemoved(unmap_notify.event())
                // } else {
                //     Event::Ignored
                // }
            }
            xcb::DESTROY_NOTIFY => {
                let destroy_event: &xcb::DestroyNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowRemoved(destroy_event.window())
            }
            xcb::ENTER_NOTIFY => {
                let enter_event: &xcb::EnterNotifyEvent = unsafe { xcb::cast_event(&event) };
                Event::WindowFocused(enter_event.event())
            }
            _ => Event::Ignored,
        }
    }
}
