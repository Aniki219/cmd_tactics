use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::Red;

use crate::{
    phases::{Phase, Transition, deal_damage::DealDamageState},
    types::{Coordinate, EntityId, EntityKind, Layer::ForegroundUi, Renderable},
    world::World,
};

pub struct PlayerAttackState {
    cat_id: EntityId, // EntityId of the cat
    origin: Coordinate,
    cursor: Renderable,
}

impl PlayerAttackState {
    pub fn new(cat_id: EntityId, origin: Coordinate, cursor_pos: Coordinate) -> Self {
        Self {
            cat_id,
            origin,
            cursor: Renderable {
                pos: cursor_pos,
                glyph: String::from("[]"),
                color: Red,
                layer: ForegroundUi,
                blink: true,
            },
        }
    }

    fn select_target(&mut self, dx: i32, dy: i32, world: &World) -> Transition {
        let dp = Coordinate(dx, dy);
        if world
            .at(self.origin.plus(dp))
            .any(|id| matches!(id.kind, EntityKind::Enemy(_)))
        {
            self.cursor.pos = self.origin.plus(dp)
        }
        Transition::None
    }
}

impl Phase for PlayerAttackState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition {
        match key.code {
            KeyCode::Char('a') => {
                let target_id = world
                    .at(self.cursor.pos)
                    .find(|id| matches!(id.kind, EntityKind::Enemy(_)))
                    .map(|e| e.id);

                if let Some(target_id) = target_id {
                    if let Some(cat) = world.players.get_mut(&self.cat_id) {
                        cat.acted = true;
                    };
                    return Transition::Push(Box::new(DealDamageState::new(
                        self.cat_id,
                        target_id,
                    )));
                }
                Transition::None
            }
            KeyCode::Char('s') => Transition::Pop,
            KeyCode::Left => self.select_target(-1, 0, world),
            KeyCode::Right => self.select_target(1, 0, world),
            KeyCode::Up => self.select_target(0, -1, world),
            KeyCode::Down => self.select_target(0, 1, world),
            _ => Transition::None,
        }
    }

    fn on_resume(&mut self, _world: &mut World) -> Transition {
        Transition::Pop
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        vec![self.cursor.clone()]
    }
}
