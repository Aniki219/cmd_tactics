use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    game::{
        Command::{self, MoveEntityBy},
        Phase,
    },
    types::*,
    world::World,
};

#[derive(Debug)]
pub struct PlayerTurnState {
    pub cursor: EntityId,
}

impl PlayerTurnState {
    pub fn new(cursor: EntityId) -> Self {
        Self { cursor }
    }

    fn move_cursor(&mut self, by_x: i32, by_y: i32) -> Option<Command> {
        Some(MoveEntityBy(self.cursor, Coordinate(by_x, by_y)))
    }

    fn select(&mut self, cursor_id: EntityId, world: &World) -> Option<Command> {
        // Get the position of the cursor if exists
        let Some(pos) = world.get(cursor_id).map(|e| e.pos) else {
            return None;
        };

        // Get the id of the Entity on the layer Entity at that position
        let target = world
            .at(pos)
            .find(|e| matches!(e.kind, EntityKind::Player))
            .map(|e| e.id);

        // Call enter_select_player if we found one
        if let Some(id) = target {
            return Some(Command::SelectPlayer(cursor_id, id));
        }

        None
    }
}

impl Phase for PlayerTurnState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Command> {
        match key.code {
            KeyCode::Char('a') => self.select(self.cursor, world),

            KeyCode::Char(c) if c.is_ascii_digit() => {
                return {
                    if let Some(digit) = c.to_digit(10) {
                        return Some(Command::MoveCursorToPlayer(
                            self.cursor,
                            EntityId(digit as i32),
                        ));
                    }
                    None
                };
            }

            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0, -1),
            KeyCode::Down => self.move_cursor(0, 1),
            _ => None,
        }
    }
}
