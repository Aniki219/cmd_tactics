use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use ratatui::style::Color::{self, White};
use strum::Display;

use crate::{
    game::Command::*,
    log,
    phases::{player_selected::PlayerSelectedState, player_turn::PlayerTurnState},
    types::{EntityKind::Cursor, Layer::ForegroundUi, *},
    world::World,
};

pub struct Game {
    pub world: World,
    pub players: Vec<EntityId>,
    pub phase: Box<dyn Phase>,
}

pub struct InitGame {}

impl InitGame {
    pub fn new() -> Self {
        Self {}
    }
}

impl Phase for InitGame {
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> VecDeque<Command> {
        VecDeque::new()
    }
}

pub trait Phase {
    fn on_enter(&mut self, _world: &mut World) {}
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> VecDeque<Command>;
}

const PLAYER_COLORS: [Color; 10] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Magenta,
    Color::LightRed,
    Color::LightYellow,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightMagenta,
];

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            phase: Box::new(InitGame::new()),
            players: Vec::new(),
        }
    }

    fn add_player(&mut self, pos: Coordinate) {
        self.players.push(self.world.spawn(
            EntityKind::Player(self.players.len() as i32 + 1),
            pos,
            Layer::Entity,
            PLAYER_COLORS[self.players.len() % PLAYER_COLORS.len()],
        ));
    }

    pub fn init(&mut self) {
        log!("Spawning players");
        self.add_player(Coordinate(4, 4));
        self.add_player(Coordinate(14, 8));
        self.add_player(Coordinate(10, 10));

        let cursor = self
            .world
            .spawn(Cursor, Coordinate::center(), ForegroundUi, White);

        self.change_phase(Box::new(PlayerTurnState::new(cursor)));
    }

    pub fn change_phase(&mut self, mut to: Box<dyn Phase>) {
        self.phase.on_exit(&mut self.world);
        to.on_enter(&mut self.world);
        self.phase = to;
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        let cmds: VecDeque<Command> = self.phase.handle_input(key, &mut self.world);

        for cmd in cmds {
            self.do_command(cmd);
        }
    }

    fn do_command(&mut self, command: Command) {
        log!("{}", command);
        match command {
            MoveEntityBy(id, by) => self.world.move_by(id, by),
            MoveEntityTo(id, to) => self.world.move_to(id, to),
            SelectPlayer(cursor, selected) => {
                self.change_phase(Box::new(PlayerSelectedState::new(cursor, selected)))
            }
            ChangePhase(phase) => {
                self.change_phase(phase);
            }
            _ => todo!(),
        };
    }
}

#[derive(Display)]
pub enum Command {
    MoveEntityBy(EntityId, Coordinate),
    MoveEntityTo(EntityId, Coordinate),
    SelectEnemy(EntityId),
    SelectPlayer(EntityId, EntityId),
    TryMove(EntityId),
    ChangePhase(Box<dyn Phase>),
}
