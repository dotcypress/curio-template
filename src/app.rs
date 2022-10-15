use crate::{
    assets::AppIcon,
    ui::{Menu, UI},
};
use curio_bsp::protocol::nec::NecCommand;
use klaptik::*;

pub enum AppEvent {
    ButtonA,
    ButtonB,
    ThumbMove(Point),
    IrCommand(NecCommand),
}

pub enum AppRequest {
    SwitchOff,
    SetBrightness(u8),
    TransmitIRCommand(NecCommand),
}

pub struct App {
    pub frame: u8,
    pub main_menu: Menu<AppIcon>,
}

impl App {
    pub fn new() -> Self {
        let main_menu = Menu::new(&[
            AppIcon::Send,
            AppIcon::Scan,
            AppIcon::Replay,
            AppIcon::Config,
        ]);
        Self {
            frame: 0,
            main_menu,
        }
    }

    pub fn invalidate(&mut self, ui: &mut UI) {
        self.frame = self.frame.wrapping_add(1);
        ui.update(self);
    }

    pub fn handle_event(&mut self, ev: AppEvent) -> Option<AppRequest> {
        match ev {
            AppEvent::ThumbMove(p) => {
                if p.y > 32 {
                    self.main_menu.move_up();
                } else if p.y < -32 {
                    self.main_menu.move_down();
                }
            }
            AppEvent::ButtonA => defmt::info!("Button A"),
            AppEvent::ButtonB => defmt::info!("Button B"),
            AppEvent::IrCommand(cmd) => {
                defmt::info!("IrCommand {} {} {}", cmd.addr, cmd.cmd, cmd.repeat)
            }
        }
        None
    }
}
