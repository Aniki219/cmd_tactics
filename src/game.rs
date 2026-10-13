use crossterm::event::KeyEvent;

use crate::{
    log,
    phases::{
        PhaseStack,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
        init_game::InitGame,
        player_turn::PlayerTurnState,
    },
    setup::{CatToml, add_cat},
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
        log!("Spawning cats");
        match load_toml::<CatToml>("data/cats.toml") {
            Ok(cat_toml) => {
                for cat_data in cat_toml.cat {
                    log!(
                        "Spawning cat {} hp: {}",
                        cat_data.name,
                        cat_data.health
                    );
                    add_cat(&mut self.world, cat_data);
                }
            }
            Err(e) => log!("Failed to load cats.toml: {}", e),
        }

        let test_string = "start";

        match test_string {
            "start" => {
                let start = self
                    .world
                    .cats
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
                            .cats
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
