pub enum ModKey<K> {
    Shift(K),
    Lock(K),
    Control(K),
    Mod1(K),
    Mod2(K),
    Mod3(K),
    Mod4(K),
    Mod5(K),
}

pub struct KeyCombo<K> {
    mod_mask: K,
    key: K
}
