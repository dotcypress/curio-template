use crate::app::App;
use klaptik::*;

mod menu;
mod widgets;

pub use menu::*;
pub use widgets::*;

pub const SPRITES: [FlashSprite; 6] = [
    FlashSprite::new(
        Asset::Background as _,
        Glyphs::Sequential(1),
        Size::new(16, 16),
        &[0; 256],
    ),
    FlashSprite::new(
        Asset::Icon as _,
        Glyphs::Sequential(8),
        Size::new(16, 16),
        include_bytes!("assets/icons.bin"),
    ),
    FlashSprite::new(
        Asset::Battery as _,
        Glyphs::Sequential(5),
        Size::new(16, 16),
        include_bytes!("assets/battery.bin"),
    ),
    FlashSprite::new(
        Asset::MenuSmall as _,
        Glyphs::Sequential(8),
        Size::new(56, 16),
        include_bytes!("assets/menu_medium.bin"),
    ),
    FlashSprite::new(
        Asset::MenuLarge as _,
        Glyphs::Sequential(8),
        Size::new(104, 32),
        include_bytes!("assets/menu_large.bin"),
    ),
    FlashSprite::new(
        Asset::Font as _,
        Glyphs::Alphabet(b"0123456789ABCDEFsx?%. "),
        Size::new(16, 24),
        include_bytes!("assets/font.bin"),
    ),
];

pub type MenuIcon = Icon<MenuItem>;

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

pub enum Asset {
    Icon = 0,
    Font = 1,
    Battery = 2,
    MenuSmall = 3,
    MenuLarge = 4,
    Background = 255,
}

impl From<Asset> for SpriteId {
    fn from(asset: Asset) -> Self {
        asset as _
    }
}

widget_mux! {
    Viewport<&App>,
    ViewportNode::MainMenu,
    {
        main_menu: MenuWidget;
        config_menu: MenuWidget;
        backlight: BacklightWidget;
        sleep_timeout: SleepTimeoutWidget;
        about: AboutWidget;
    },
    |widget: &mut Viewport, state: &App| {
        widget.about.update(state);
        widget.main_menu.update(&state.main_menu);
        widget.config_menu.update(&state.config_menu);
        widget.backlight.update(state.options.backlight);
        widget.sleep_timeout.update(state.options.sleep_timeout);
        widget.set_active(state.active_widget);
    }
}
