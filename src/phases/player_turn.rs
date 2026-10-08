use crossterm::event::{KeyCode, KeyEvent};

use crate::{
   phases::{Phase, player_selected::PlayerSelectedState}, types::*, world::World
};

#[derive(Debug)]
pub struct PlayerTurnState {
    pub cursor: EntityId,
}

impl PlayerTurnState {
    pub fn new(cursor: EntityId) -> Self {
        Self { cursor }
    }

    fn select(&mut self, cursor_id: EntityId, world: &World) -> Option<Box<dyn Phase>> {
        // Get the position of the cursor if exists
        let pos = world.get(cursor_id)?.pos;

        // Get the id of the Entity on the layer Entity at that position
        let target = world
            .at(pos)
            .find(|e| matches!(e.kind, EntityKind::Player(_)))
            .map(|e| e.id);

        // Call enter_select_player if we found one
        if let Some(id) = target {
            return Some(Box::new(PlayerSelectedState::new(cursor_id, id)));
        }

        None
    }
}

impl Phase for PlayerTurnState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>> {
        let mut move_cursor = |x, y| {
            world.move_by(self.cursor, Coordinate(x, y));
            return None;
        };
        
        match key.code {
            KeyCode::Char('a') => self.select(self.cursor, world),

            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    let id = EntityId(digit as i32);
                    if let Some(to) = world.get(id).map(|e| e.pos) {
                        world.move_to(self.cursor, to);
                        return Some(Box::new(PlayerSelectedState::new(self.cursor, id)));
                    }
                }
                None
            }

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => None,
        }
    }
}
