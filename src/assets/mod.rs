#![allow(non_upper_case_globals)]

use klaptik::*;

pub type GlyphIcon = RomIcon<Glyph>;
pub type MenuIcon = RomIcon<MenuItem>;
pub type SpriteLabel<const LEN: usize> = Label<RomSprite, LEN>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Curio = 0,
    Send = 1,
    Scan = 2,
    Replay = 3,
    Config = 4,
    Sleep = 5,
    Backlight = 6,
    About = 7,
}

impl From<MenuItem> for Glyph {
    fn from(item: MenuItem) -> Self {
        item as _
    }
}

pub const IconSprite: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(16, 16),
    include_bytes!("icons.bin"),
);

pub const BatterySprite: RomSprite = RomSprite::new(
    Glyphs::Sequential(5),
    Size::new(16, 16),
    include_bytes!("battery.bin"),
);

pub const MenuMediumSprite: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(56, 16),
    include_bytes!("menu_medium.bin"),
);

pub const MenuLargeSprite: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(104, 32),
    include_bytes!("menu_large.bin"),
);

pub const FontSprite: RomSprite = RomSprite::new(
    Glyphs::Alphabet(b"0123456789ABCDEFsx?%. "),
    Size::new(16, 24),
    include_bytes!("font_24.bin"),
);
