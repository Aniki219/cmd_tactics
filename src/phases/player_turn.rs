use std::collections::VecDeque;

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

    fn move_cursor(&mut self, by_x: i32, by_y: i32) -> VecDeque<Command> {
        let mut cmd = VecDeque::new();
        cmd.push_back(MoveEntityBy(self.cursor, Coordinate(by_x, by_y)));
        cmd
    }

    fn select(&mut self, cursor_id: EntityId, world: &World) -> VecDeque<Command> {
        let mut cmd = VecDeque::new();

        // Get the position of the cursor if exists
        let Some(pos) = world.get(cursor_id).map(|e| e.pos) else {
            return cmd;
        };

        // Get the id of the Entity on the layer Entity at that position
        let target = world
            .at(pos)
            .find(|e| matches!(e.kind, EntityKind::Player(_)))
            .map(|e| e.id);

        // Call enter_select_player if we found one
        if let Some(id) = target {
            cmd.push_back(Command::SelectPlayer(cursor_id, id));
        }

        cmd
    }
}

impl Phase for PlayerTurnState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> VecDeque<Command> {
        match key.code {
            KeyCode::Char('a') => self.select(self.cursor, world),

            KeyCode::Char(c) if c.is_ascii_digit() => {
                let cmd = VecDeque::new();

                if let Some(digit) = c.to_digit(10) {
                    if let Some(to) = world.get(EntityId(digit as i32)).map(|e| e.pos) {
                        world.move_to(self.cursor, to);
                    }
                }

                cmd
            }

            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0, -1),
            KeyCode::Down => self.move_cursor(0, 1),
            _ => VecDeque::new(),
        }
    }
}
