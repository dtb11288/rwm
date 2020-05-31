pub mod xcb_keys;

pub trait KeyConvert {
    type KeyCombo;
    fn convert(&self, key: KeyCombo) -> Self::KeyCombo;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ModKey {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Key(pub char);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyCombo {
    pub mod_keys: Vec<ModKey>,
    pub key: Key
}
