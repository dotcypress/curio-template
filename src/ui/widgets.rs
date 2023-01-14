use crate::app::App;
use crate::ui::*;
use core::fmt::Write;

widget!(
    Background<()>,
    Tile<Glyph>,
    Asset::Background, 0,
    Point::zero(),
    Size::new(16, 16),
    16, 8;
);


widget_group! {
    BacklightWidget<u8>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Backlight, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Backlight, Point::new(24, 0);
        brightness: Label<4>, Asset::Font, "100%", Point::new(24, 24), Size::new(16, 24);
    },
    |widget: &mut BacklightWidget, brightness: u8| {
        write!(widget.brightness, "{: >3}%",  brightness * 10).unwrap();
    }
}

widget_group! {
    SleepTimeoutWidget<u8>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Sleep, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Sleep, Point::new(24, 0);
        value: Label<3>, Asset::Font, "10s", Point::new(32, 24), Size::new(16, 24);
    },
    |widget: &mut SleepTimeoutWidget, timeout: u8| {
        write!(widget.value, "{timeout}s").unwrap();
    }
}

widget_group! {
    AboutWidget<&App>,
    {
        bg: Background;
        icon: MenuIcon, Asset::Icon, MenuItem::Curio, Point::zero();
        title: MenuIcon, Asset::MenuSmall, MenuItem::Curio, Point::new(24, 0);
        battery: GlyphIcon, Asset::Battery, 0, Point::new(112, 0);
    },
    |widget: &mut AboutWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
    }
}
