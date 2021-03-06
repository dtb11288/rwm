use crate::window::{Window, WindowType, Geometry};
use std::fmt::Debug;
use crate::keys::KeyCombo;
use std::hash::Hash;
use crate::config::Config;
use futures::Stream;

pub mod xcb_server;

#[derive(Debug, Eq, PartialEq)]
pub enum Event<W, K> {
    DisplayInited,
    ScreenAdded(W, Geometry),
    KeyPressed(K),
    WindowAdded(W, WindowType),
    WindowRemoved(W),
    WindowFocused(W),
    DisplayEnded,
    Ignored,
}

pub trait DisplayServer: Stream<Item=Event<<Self as DisplayServer>::Window, <Self as DisplayServer>::KeyCombo>> + Clone {
    type Window: Debug + Clone + Eq;
    type KeyCombo: From<KeyCombo> + Hash + Eq + Debug;
    fn new(config: &Config) -> Self;
    fn configure_window(&self, window: &Window<Self::Window>);
    fn set_visibility(&self, window: &Self::Window, show: bool);
    fn quit(&self);
}
