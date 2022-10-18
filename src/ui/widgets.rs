use crate::app::App;
use crate::assets::*;
use core::fmt::Write;
use curio_bsp::Display;
use klaptik::*;

widget_group! {
    BacklightWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Backlight, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Backlight, Point::new(24, 0);
        brightness: SpriteLabel<4>, FontSprite, "100%", Point::new(24, 24);
    },
    |widget: &mut BacklightWidget, brightness: u8| {
        write!(widget.brightness, "{: >3}%",  brightness * 10).unwrap();
    }
}

widget_group! {
    SleepTimeoutWidget<u8>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Sleep, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Sleep, Point::new(24, 0);
        value: SpriteLabel<3>, FontSprite, "10s", Point::new(32, 24);
    },
    |widget: &mut SleepTimeoutWidget, timeout: u8| {
        write!(widget.value, "{timeout}s").unwrap();
    }
}

widget_group! {
    AboutWidget<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: MenuIcon, IconSprite, MenuItem::Curio, Point::zero();
        title: MenuIcon, MenuMediumSprite, MenuItem::Curio, Point::new(24, 0);
        battery: GlyphIcon, BatterySprite, 0, Point::new(112, 0);
    },
    |widget: &mut AboutWidget, state: &App| {
        widget.battery.update(state.battery_voltage);
    }
}
