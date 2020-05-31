use crate::keys::{KeyConvert, Key, KeyCombo, ModKey};

pub struct XcbKeyConvert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct XcbKeyCombo {
    pub mod_mask: u32,
    pub key: u32,
}

impl From<ModKey> for u32 {
    fn from(mod_key: ModKey) -> Self {
        match mod_key {
            ModKey::Shift => xcb::MOD_MASK_SHIFT,
            ModKey::Lock => xcb::MOD_MASK_LOCK,
            ModKey::Control => xcb::MOD_MASK_CONTROL,
            ModKey::Mod1 => xcb::MOD_MASK_1,
            ModKey::Mod2 => xcb::MOD_MASK_2,
            ModKey::Mod3 => xcb::MOD_MASK_3,
            ModKey::Mod4 => xcb::MOD_MASK_4,
            ModKey::Mod5 => xcb::MOD_MASK_5,
        }
    }
}

impl From<Key> for u32 {
    fn from(key: Key) -> Self {
        key.0 as u32
    }
}

impl KeyConvert for XcbKeyConvert {
    type KeyCombo = XcbKeyCombo;

    fn convert(&self, key: KeyCombo) -> Self::KeyCombo {
        let mod_mask = key.mod_keys
            .into_iter()
            .fold(0, |mask, mod_key| {
                let mod_mask: u32 = mod_key.into();
                mask | mod_mask
            });
        let key = key.key.into();
        XcbKeyCombo { mod_mask, key }
    }
}
