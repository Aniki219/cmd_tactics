use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    phases::{Phase, player_turn::PlayerTurnState},
    types::{EntityKind::MoveTile, Layer::BackgroundUi, *},
    world::World,
};

#[derive(Debug)]
pub struct PlayerSelectedState {
    cursor: EntityId,
    selected: EntityId,
    tiles: Vec<EntityId>,
}

impl PlayerSelectedState {
    pub fn new(cursor: EntityId, selected: EntityId) -> Self {
        Self {
            cursor,
            selected,
            tiles: Vec::new(),
        }
    }

    fn back(&self) -> Box<dyn Phase> {
        Box::new(PlayerTurnState::new(self.cursor))
    }

    fn confirm(&mut self, world: &mut World) -> Option<Box<dyn Phase>> {
        {
            let pos = world.get(self.cursor)?.pos;

            if world.at(pos).any(|e| matches!(e.layer, Layer::Entity)) {
                return None;
            };
            if world.at(pos).any(|e| matches!(e.kind, EntityKind::MoveTile))
            {
                world.move_to(self.selected, pos);
            };

            Some(self.back())
        }
    }
}

impl Phase for PlayerSelectedState {
    fn on_enter(&mut self, world: &mut World) {
        let Some(origin) = world.get(self.selected).map(|e| e.pos) else {
            return;
        };
        for i in -3..=3_i32 {
            for j in -3..=3_i32 {
                if i.abs() + j.abs() < 4 && !(i == 0 && j == 0) {
                    let id =
                        world.spawn(MoveTile, origin.plus(Coordinate(i, j)), BackgroundUi, White);
                    self.tiles.push(id);
                }
            }
        }
    }

    fn on_exit(&mut self, world: &mut World) {
        for id in self.tiles.drain(..) {
            world.despawn(id);
        }
    }

    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>> {
        let mut move_cursor = |x, y| {
            world.move_by(self.cursor, Coordinate(x, y));
            return None;
        };

        match key.code {
            // Pressing A on a MoveTile should move you there
            // on an Entity should do nothing
            // on an empty tile should cancel the move command and return to PlayerTurn
            KeyCode::Char('a') => self.confirm(world),
            KeyCode::Char('s') => Some(self.back()),

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => None,
        }
    }
}
