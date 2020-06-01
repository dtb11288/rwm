pub mod xcb_keys;

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

impl From<String> for ModKey {
    fn from(key: String) -> Self {
        match key.as_str() {
            "shift" => ModKey::Shift,
            "ctrl" => ModKey::Control,
            "lock" => ModKey::Lock,
            "mod1" => ModKey::Mod1,
            "mod2" => ModKey::Mod2,
            "mod3" => ModKey::Mod3,
            "mod4" => ModKey::Mod4,
            "mod5" => ModKey::Mod5,
            _ => panic!("Invalid modifier key")
        }
    }
}
