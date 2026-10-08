use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    game::{
        Command::{self, MoveEntityBy},
        Phase,
    },
    phases::player_turn::PlayerTurnState,
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

    fn move_cursor(&mut self, by_x: i32, by_y: i32) -> VecDeque<Command> {
        let mut cmd = VecDeque::<Command>::new();
        cmd.push_back(MoveEntityBy(self.cursor, Coordinate(by_x, by_y)));

        cmd
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

    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> VecDeque<Command> {
        match key.code {
            // Pressing A on a MoveTile should move you there
            // on an Entity should do nothing
            // on an empty tile should cancel the move command and return to PlayerTurn
            KeyCode::Char('a') => {
                let mut cmd = VecDeque::<Command>::new();

                let Some(pos) = world.get(self.cursor).map(|e| e.pos) else {
                    return cmd;
                };
                let entities: Vec<&Entity> = world.at(pos).collect();
                if let Some(_) = entities.iter().find(|e| matches!(e.layer, Layer::Entity)) {
                    return cmd;
                };
                if let Some(_) = entities
                    .iter()
                    .find(|e| matches!(e.kind, EntityKind::MoveTile))
                {
                    cmd.push_back(Command::MoveEntityTo(self.selected, pos));
                };
                cmd.push_back(Command::ChangePhase(Box::new(PlayerTurnState::new(
                    self.cursor,
                ))));

                cmd
            }

            KeyCode::Char('s') => {
                let mut cmd = VecDeque::new();
                cmd.push_back(Command::ChangePhase(Box::new(PlayerTurnState::new(
                    self.cursor,
                ))));
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
