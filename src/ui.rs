use crate::app::App;
use crate::assets::*;
use curio_bsp::Display;
use klaptik::*;

pub struct Menu<G: Into<Glyph> + 'static> {
    lines: &'static [G],
    cursor: usize,
}

impl<G: Into<Glyph> + Copy> Menu<G> {
    pub fn new(lines: &'static [G]) -> Self {
        Self { lines, cursor: 0 }
    }

    pub fn move_up(&mut self) {
        self.cursor = if self.cursor == 0 {
            self.lines.len() - 1
        } else {
            self.cursor - 1
        }
    }

    pub fn move_down(&mut self) {
        self.cursor = (self.cursor + 1) % self.lines.len();
    }

    pub fn active(&self) -> G {
        self.lines[(self.cursor + 1) % self.lines.len()]
    }
}

widget_group! {
    MenuWindow<&Menu<AppIcon>>,
    {
        bg: Background, Point::zero(), Size::new(132, 64);
        icon1: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 0);
        line1: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 0);
        icon2: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(2, 24);
        line2: RomIcon<AppIcon>, MenuLarge, AppIcon::About, Point::new(24, 16);
        icon3: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::new(0, 48);
        line3: RomIcon<AppIcon>, MenuMedium, AppIcon::About, Point::new(24, 48);
    },
    |nodes: &mut MenuWindow, state: &Menu<AppIcon>| {
        let mut lines = state.lines.iter().cycle().skip(state.cursor);
        let line = lines.next().unwrap();
        nodes.icon1.update(*line);
        nodes.line1.update(*line);

        let line = lines.next().unwrap();
        nodes.icon2.update(*line);
        nodes.line2.update(*line);

        let line = lines.next().unwrap();
        nodes.icon3.update(*line);
        nodes.line3.update(*line);
    }
}

widget_group! {
    ConfigWindow<&App>,
    {
        bg: Background, Point::zero(), Display::SIZE;
        icon: RomIcon<AppIcon>, Icon16, AppIcon::Config, Point::zero();
    }
}

widget_group!(AboutWindow<()>, {
    bg: Background, Point::zero(), Display::SIZE;
    icon: RomIcon<AppIcon>, Icon16, AppIcon::About, Point::zero();
});

widget_mux! {
    UI<&App>,
    UiNode::MainMenu,
    {
        main_menu: MenuWindow;
        about: AboutWindow;
        config: ConfigWindow;
    },
    |ui: &mut UI, state: &App| {
        ui.config.update(state);
        ui.main_menu.update(&state.main_menu);
    }
}
