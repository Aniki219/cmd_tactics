use crossterm::event::KeyEvent;
use ratatui::style::Color::{self, White};
use strum::Display;

use crate::{
    game::Command::{MoveCursorToPlayer, MoveEntityBy},
    log,
    phases::{player_selected::PlayerSelectedState, player_turn::PlayerTurnState},
    types::{EntityKind::Cursor, Layer::ForegroundUi, *},
    world::World,
};

pub struct Game {
    pub world: World,
    pub phase: Option<Box<dyn Phase>>,
}

pub trait Phase {
    fn on_enter(&mut self, _world: &mut World) {}
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Command>;
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            phase: None,
        }
    }

    pub fn init(&mut self) {
        log!("Spawning players");
        self.world.spawn(
            EntityKind::Player,
            Coordinate(4, 4),
            Layer::Entity,
            Color::Blue,
        );
        self.world.spawn(
            EntityKind::Player,
            Coordinate(10, 10),
            Layer::Entity,
            Color::Green,
        );
        self.world.spawn(
            EntityKind::Player,
            Coordinate(14, 12),
            Layer::Entity,
            Color::Red,
        );

        let cursor = self
            .world
            .spawn(Cursor, Coordinate::center(), ForegroundUi, White);

        self.change_phase(Box::new(PlayerTurnState::new(cursor)));
    }

    pub fn change_phase(&mut self, mut to: Box<dyn Phase>) {
        if let Some(mut old) = self.phase.take() {
            old.on_exit(&mut self.world);
        }
        to.on_enter(&mut self.world);
        self.phase = Some(to);
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        let cmd = self
            .phase
            .as_mut()
            .and_then(|p| p.handle_input(key, &mut self.world));

        if let Some(cmd) = cmd {
            self.do_command(cmd);
        }
    }

    fn do_command(&mut self, command: Command) {
        log!("{}", command);
        match command {
            MoveEntityBy(id, by) => self.world.move_by(id, by),
            MoveCursorToPlayer(cursor, id) => {
                let Some(pos) = self.world.get(id).map(|e| e.pos) else {
                    return;
                };
                self.world.move_to(cursor, pos);
            }
            Command::SelectPlayer(cursor, selected) => {
                self.change_phase(Box::new(PlayerSelectedState::new(cursor, selected)))
            }
            _ => todo!(),
        };
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum Command {
    MoveEntityBy(EntityId, Coordinate),
    MoveCursorToPlayer(EntityId, EntityId),
    SelectEnemy(EntityId),
    SelectPlayer(EntityId, EntityId),
    TryMove(EntityId),
}
