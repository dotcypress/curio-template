use crate::ui::UI;
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
}

impl App {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn invalidate(&mut self, ui: &mut UI) {
        self.frame = self.frame.wrapping_add(1);
        ui.update(self);
    }

    pub fn handle_event(&mut self, _ev: AppEvent) -> Option<AppRequest> {
        None
    }
}
