use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    game::{
        Command::{self, MoveEntityBy},
        Phase,
    },
    types::{EntityKind::MoveTile, Layer::ForegroundUi, *},
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

    fn move_cursor(&mut self, by_x: i32, by_y: i32) -> Option<Command> {
        Some(MoveEntityBy(self.cursor, Coordinate(by_x, by_y)))
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
                        world.spawn(MoveTile, origin.plus(Coordinate(i, j)), ForegroundUi, White);
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

    fn handle_input(&mut self, key: KeyEvent, _world: &mut World) -> Option<Command> {
        match key.code {
            KeyCode::Char('A') => Some(Command::TryMove(self.cursor)),

            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0, -1),
            KeyCode::Down => self.move_cursor(0, 1),
            _ => None,
        }
    }
}
