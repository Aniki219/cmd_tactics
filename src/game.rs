use crossterm::event::KeyEvent;
use ratatui::Frame;
use serde::{Deserialize};

use crate::{
    log,
    phases::{
        PhaseStack,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
        enemy_turn::EnemyTurnState,
        init_game::InitGame,
        player_turn::PlayerTurnState,
    },
    setup::{CatData, add_enemy_cat, add_player_cat},
    types::{Coordinate, EntityId},
    ui::game_ui::GameUi,
    util::load::load_toml,
    world::World,
};

pub struct Game {
    pub world: World,
    pub phases: PhaseStack,
    pub ui: GameUi,
}

#[derive(Deserialize)]
pub struct MissionReward {
    money: i32,
    xp: i32,
}

#[derive(Deserialize)]
pub struct PlayerSpawn {
    slot: usize,
    pos: Coordinate,
}

#[derive(Deserialize)]
pub struct EnemySpawn {
    name: String,
    pos: Coordinate,
}

#[derive(Deserialize)]
pub struct MissionData {
    name: String,
    reward: MissionReward,
    players: Vec<PlayerSpawn>,
    enemies: Vec<EnemySpawn>,
}

#[derive(Deserialize)]
pub struct PlayerCatsFileData {
    cats: Vec<PlayerCatData>,
}

#[derive(Deserialize)]
pub struct PlayerCatData {
    id: i32,
    name: String,
    xp: i32,
}

#[derive(Deserialize)]
pub struct CatsFileData {
    cats: Vec<CatData>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let phases = PhaseStack::new(Box::new(InitGame::new()), &mut world);
        let ui = GameUi {};
        Self { world, phases, ui }
    }

    fn spawn_mission_cats(&mut self) -> Result<(), String> {
        let slots = vec![1, 2, 3];
        let cat_data = load_toml::<CatsFileData>("data/cats.toml")
            .map(|f| f.cats)
            .expect("Failed to read cats data from file");
        let player_cats = load_toml::<PlayerCatsFileData>("data/player_cats.toml")
            .map(|f| f.cats)
            .expect("Failed to read player cats data from file");
        let mission_data = load_toml::<MissionData>("data/missions/mission_01.toml")
            .expect("Failed to read mission data from file");

        for spawn in &mission_data.players {
            let cat_id = slots
                .get(spawn.slot - 1)
                .ok_or_else(|| format!("mission slot {} has no assigned cat", spawn.slot))?;

            let player_cat = player_cats
                .iter()
                .find(|c| c.id == *cat_id)
                .ok_or_else(|| format!("player cat id {} not found", cat_id))?;

            let base_stats = cat_data
                .iter()
                .find(|c| c.name == player_cat.name)
                .ok_or_else(|| format!("no cat definition named '{}'", player_cat.name))?;

            log!("Spawning cat {}", base_stats.name);
            add_player_cat(&mut self.world, spawn.pos, base_stats);
        }

        for spawn in &mission_data.enemies {
            let base_stats = cat_data
                .iter()
                .find(|c| c.name == spawn.name)
                .ok_or_else(|| format!("no cat definition named '{}'", spawn.name))?;

            log!("Spawning enemy {}", spawn.name);
            add_enemy_cat(&mut self.world, spawn.pos, base_stats);
        }

        Ok(())
    }

    pub fn init(&mut self) {
        log!("Spawning cats");
        let res = self.spawn_mission_cats();
        if res.is_err() {
            log!(
                "Spawn mission cats errored {}",
                res.err().unwrap_or_default()
            )
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
        let Some(phase) = self.phases.stack.last_mut() else {
            return;
        };
        // as_ref pulls it out of the Box
        self.ui.render(phase.as_mut(), &self.world, frame);
    }
}
