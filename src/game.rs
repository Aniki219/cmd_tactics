use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{
    log,
    model::{
        common::Coordinate,
        defs::{load::load_defs, mission::MissionData},
        transient::{game::GameData, mission::spawn_mission, world::World},
    },
    phases::{
        PhaseStack, action_menu::ActionMenuState, enemy_turn::EnemyTurnState, init_game::InitGame,
        player_turn::PlayerTurnState,
    },
    ui::game_ui::GameUi,
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
        if let Err(err) = self.load_mission() {
            log!("Failed to load mission {}", err);
            return;
        }
        self.start_phase();
    }

    fn load_mission(&mut self) -> Result<(), String> {
        let game_data = GameData::load()?;
        let mission_data = load_defs::<MissionData>("missions/mission_01.toml")
            .map_err(|e| format!("mission_01.toml: {e:?}"))?;

        log!("Spawning mission");
        let loadout = [1, 2, 3];
        spawn_mission(&mut self.world, &game_data, &mission_data, &loadout)
    }

    fn start_phase(&mut self) {
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
                let Some(id) = self
                    .world
                    .players
                    .iter()
                    .min_by_key(|k| k.0)
                    .map(|(id, _)| *id)
                else {
                    log!("No players");
                    return;
                };
                match ActionMenuState::new(id, &self.world) {
                    Ok(state) => {
                        self.phases.reset(Box::new(state), &mut self.world);
                    }
                    Err(e) => {
                        log!(e);
                    }
                }
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
