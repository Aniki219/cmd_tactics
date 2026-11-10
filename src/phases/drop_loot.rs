use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame};

use crate::{
    phases::{Phase, Transition, mission_complete::MissionCompleteState, player_turn::PlayerTurnState}, types::{Cat, Coordinate, EntityId}, ui::{
        game_ui::{Panels, UiTheme},
        text_cycler::{CyclerEvent::Finished, TextCycler},
    }, world::World,
};

pub struct DropLootState {
    target_id: EntityId,
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
            text,
        }
    }

    fn check_battle_end(&mut self, world: &mut World) -> Option<Transition> {
        if world.enemies.len() <= 0 {
            return Some(Transition::Reset(Box::new(MissionCompleteState::new())))
        }
        let pos = world
            .get(self.target_id)
            .map_or(Coordinate::center(), |t| t.pos);
        return Some(Transition::Reset(Box::new(PlayerTurnState::new(pos))));
    }
}

impl Phase for DropLootState {
    fn on_enter(&mut self, world: &mut World) -> Transition {
        if let Some(target) = world.get_cat(&self.target_id)
            && target.health <= 0
        {
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
