use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{
    log,
    phases::{
        Phase, PhaseStack,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
        enemy_turn::EnemyTurnState,
        init_game::InitGame,
        player_turn::PlayerTurnState,
    },
    setup::{CatToml, add_enemy_cat, add_player_cat},
    types::{Coordinate, EntityId},
    ui::GameUi,
    util::load::load_toml,
    world::World,
};

pub struct Game {
    pub world: World,
    pub phases: PhaseStack,
    pub ui: GameUi,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let phases = PhaseStack::new(Box::new(InitGame::new()), &mut world);
        let ui = GameUi {};
        Self { world, phases, ui }
    }

    pub fn init(&mut self) {
        log!("Spawning cats");
        match load_toml::<CatToml>("data/cats.toml") {
            Ok(cat_toml) => {
                for cat_data in cat_toml.cat {
                    log!("Spawning cat {} hp: {}", cat_data.name, cat_data.health);
                    add_player_cat(&mut self.world, cat_data);
                }
                for cat_data in cat_toml.enemy {
                    log!("Spawning enemy {} hp: {}", cat_data.name, cat_data.health);
                    add_enemy_cat(&mut self.world, cat_data);
                }
            }
            Err(e) => log!("Failed to load cats.toml: {}", e),
        }

        let test_string = "enemy";

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
            "enemy" => {
                self.phases
                    .reset(Box::new(EnemyTurnState::new(&self.world)), &mut self.world);
            }
            _ => {
                log!("No test_string to pick phase");
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        self.phases.handle_input(key, &mut self.world);
    }

    pub fn tick(&mut self) {
        self.phases.tick(&mut self.world);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        // phase is &Box<dyn Phase>
        let Some(phase) = self.phases.stack.last() else {
            return;
        };
        // as_ref pulls it out of the Box
        self.ui.render(phase.as_ref(), &self.world, frame);
    }
}
