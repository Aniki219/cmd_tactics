use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,

};

use crate::{
    model::{
        common::Coordinate, transient::{
            cat::Cat, entity::EntityId, world::World,
        },
    }, phases::{
        Phase, Transition, UiTheme, mission_complete::MissionCompleteState, player_turn::PlayerTurnState,
    }, ui::{game_ui::Panels, text_cycler::{CyclerEvent::Finished, TextCycler}},
};

pub struct DropLootState {
    target_id: EntityId,
    target_pos: Coordinate,
    text: TextCycler,
}

impl DropLootState {
    pub fn new(source: &Cat, target: &Cat) -> Self {
        let text = TextCycler::new(vec![
            format!("{} was defeated!", target.name),
            format!("{} gained {} experience", source.name, 10),
        ]);
        Self {
            target_id: target.id,
            target_pos: Coordinate::default(),
            text,
        }
    }

    fn check_battle_end(&mut self, world: &mut World) -> Option<Transition> {
        if world.enemies.len() <= 0 {
            return Some(Transition::Reset(Box::new(MissionCompleteState::new())));
        }
        return Some(Transition::Reset(Box::new(PlayerTurnState::new(
            self.target_pos,
        ))));
    }
}

impl Phase for DropLootState {
    fn on_enter(&mut self, world: &mut World) -> Transition {
        if let Some(target) = world.get(self.target_id) {
            self.target_pos = target.pos;
            world.despawn(self.target_id);
        }
        Transition::None
    }

    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition {
        match key.code {
            KeyCode::Char('a') => {
                if matches!(self.text.advance(), Finished) {
                    if let Some(value) = self.check_battle_end(world) {
                        return value;
                    }
                }
            }
            _ => (),
        };
        Transition::None
    }

    fn theme(&self) -> UiTheme {
        UiTheme::focus_hud()
    }

    fn render(&mut self, _world: &World, frame: &mut Frame, panels: &mut Panels) {
        self.text.render(frame, panels.hud_area());
    }
}
