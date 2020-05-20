pub trait KeyConvert {
    type KeyType;
    fn backward(&self, key: Self::KeyType) -> Key;
    fn forward(&self, keys: Vec<Key>) -> Self::KeyType;
}

pub enum Key {
    ModKey(ModKey),
    Key(String)
}

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

pub struct KeyCombo {
    mod_key: K,
    key: K
}
