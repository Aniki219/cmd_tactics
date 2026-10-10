use crossterm::event::KeyEvent;

use crate::{
    log,
    phases::{
        Phase,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
        init_game::InitGame,
        player_turn::PlayerTurnState,
    },
    setup::{PlayerToml, add_player},
    types::{Coordinate, EntityId},
    util::load::load_toml,
    world::World,
};

pub struct Game {
    pub world: World,
    pub phase: Box<dyn Phase>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            phase: Box::new(InitGame::new()),
        }
    }

    pub fn init(&mut self) {
        log!("Spawning players");
        match load_toml::<PlayerToml>("data/players.toml") {
            Ok(player_toml) => {
                for player_data in player_toml.player {
                    log!(
                        "Spawning player {} hp: {}",
                        player_data.name,
                        player_data.health
                    );
                    add_player(&mut self.world, player_data);
                }
            }
            Err(e) => log!("Failed to load players.toml: {}", e),
        }

        let test_string = "action";

        match test_string {
            "start" => {
                let start = self
                    .world
                    .players
                    .first()
                    .and_then(|player| self.world.get(player.id))
                    .map(|e| e.pos)
                    .unwrap_or_else(Coordinate::center);
                self.change_phase(Box::new(PlayerTurnState::new(start)));
            }
            "action" => {
                if let Some(data) = self.world.players.first().map(|p| p.data.clone()) {
                    let prev_pos = data.pos.plus(Coordinate(0, -2));
                    self.change_phase(Box::new(ActionMenuState::new(
                        data.pos,
                        EntityId(1),
                        data,
                        prev_pos,
                        &ACTION_MENU_ITEMS,
                    )));
                }
            }
            _ => {
                log!("No test_string to pick phase");
            }
        }
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
