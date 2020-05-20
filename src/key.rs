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
    mod_mask: ModKey,
    key: String,
}
