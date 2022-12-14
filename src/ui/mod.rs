use crate::app::App;
use klaptik::*;

mod menu;
mod widgets;

pub use menu::*;
pub use widgets::*;

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
