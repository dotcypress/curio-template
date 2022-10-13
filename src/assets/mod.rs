use klaptik::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Curio {
    About = 0,
    Config = 1,
}

impl Into<Glyph> for Curio {
    fn into(self) -> Glyph {
        self as Glyph
    }
}

#[allow(non_upper_case_globals)]
pub const Icons: RomSprite = RomSprite::new(
    Glyphs::Sequential(8),
    Size::new(16, 16),
    include_bytes!("icons.bin"),
);
