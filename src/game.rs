use crossterm::event::KeyEvent;

use crate::{
    log,
    phases::{Phase, init_game::InitGame, player_turn::PlayerTurnState},
    setup::{PlayerData, add_player},
    types::{Coordinate, EntityId},
    util::load::load_toml,
    world::World,
};

pub struct Game {
    pub world: World,
    pub players: Vec<EntityId>,
    pub phase: Box<dyn Phase>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            phase: Box::new(InitGame::new()),
            players: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        log!("Spawning players");
        match load_toml::<PlayerData>("data/players.toml") {
            Ok(player_data) => {
                for player in player_data.player {
                    log!("Spawning player {} hp: {}", player.name, player.health);
                    add_player(&mut self.world, &mut self.players, player.pos);
                }
            }
            Err(e) => log!("Failed to load players.toml: {}", e),
        }
        let start = self
            .players
            .first()
            .and_then(|&id| self.world.get(id))
            .map(|e| e.pos)
            .unwrap_or_else(Coordinate::center);
        self.change_phase(Box::new(PlayerTurnState::new(start)));
    }

    pub fn change_phase(&mut self, mut to: Box<dyn Phase>) {
        self.phase.on_exit(&mut self.world);
        to.on_enter(&mut self.world);
        self.phase = to;
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if let Some(next) = self.phase.handle_input(key, &mut self.world) {
            self.change_phase(next);
        }
    }
}
