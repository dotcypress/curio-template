use crate::app::App;
use crate::assets::*;
use curio_bsp::Display;
use klaptik::*;

widget_group! {
    UI<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<Glyph>, Icons, 0, Point::zero();
        window: WindowMux;
    },
    |group: &mut UI, state: &App| {
        group.icon.update((state.frame / 20) % 8);
        group.window.update(state);
    }
}

widget_mux! {
    WindowMux<&App>,
    WindowMuxNode::About,
    {
        about: AboutWindow;
        config: ConfigWindow;
    },
    |mux: &mut WindowMux, state: &App| {
        mux.config.update(state);
        // mux.set_active(WindowMuxNode::Config);
    }
}

widget_group! {
    ConfigWindow<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<Curio>, Icons, Curio::Config, Point::zero();
    }
}

widget_group!(AboutWindow<()>, {
    bg: Background, Point::zero(), Display::SIZE;
    icon: RomIcon<Curio>, Icons, Curio::About, Point::zero();
});
