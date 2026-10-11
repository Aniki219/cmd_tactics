use crossterm::event::KeyEvent;

use crate::{
    log,
    phases::{
        PhaseStack,
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
    pub phases: PhaseStack,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let phases = PhaseStack::new(Box::new(InitGame::new()), &mut world);
        Self { world, phases }
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

        let test_string = "start";

        match test_string {
            "start" => {
                let start = self
                    .world
                    .players
                    .iter()
                    .min_by_key(|k| k.0)
                    .and_then(|(id, _)| self.world.get(*id))
                    .map(|e| e.pos)
                    .unwrap_or_else(Coordinate::center);
                self.phases
                    .reset(Box::new(PlayerTurnState::new(start)), &mut self.world);
            }
            "action" => {
                self.phases.reset(
                    Box::new(ActionMenuState::new(
                        self.world
                            .players
                            .iter()
                            .min_by_key(|k| k.0)
                            .map(|(id, _)| *id)
                            .unwrap_or_else(|| EntityId(1)),
                        &ACTION_MENU_ITEMS,
                    )),
                    &mut self.world,
                );
            }
            _ => {
                log!("No test_string to pick phase");
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        self.phases.handle_input(key, &mut self.world);
    }
}
