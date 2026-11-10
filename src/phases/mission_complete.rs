use ratatui::{Frame, style::Color::Blue};

use crate::{
    phases::{Phase, Transition}, ui::{
        game_ui::{Panels, PopupState, UiTheme}, text_cycler::{TextCycler},
    }, util::timer::Timer, world::World,
};

pub struct MissionCompleteState {
    timer: Timer,
    text: TextCycler,
}

impl MissionCompleteState {
    pub fn new() -> Self {
        let text = TextCycler::new(vec![
            format!("Enemies were defeated!"),
        ]);
        Self {text, timer: Timer::new(10)}
    }
}

impl Phase for MissionCompleteState {
    fn on_tick(&mut self, _world: &mut World) -> Transition {
        Transition::None
    }

    fn theme(&self) -> UiTheme {
        UiTheme::focus_hud()
    }

    fn render(&mut self, _world: &World, frame: &mut Frame, panels: &mut Panels) {
        self.text.render(frame, panels.hud_area());
        panels.popup(frame, PopupState{
            title: String::from("Battle End"),
            color: Blue,
            body: String::from("Mission Complete!")
        });
    }
}
