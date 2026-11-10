use serde::Deserialize;

use crate::types::{Abilities, Coordinate, Experience};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct MissionReward {
    pub money: i32,
    pub xp: i32,
}

#[derive(Deserialize)]
pub struct PlayerSpawn {
    pub slot: usize,
    pub pos: Coordinate,
}

#[derive(Deserialize)]
pub struct EnemySpawn {
    pub name: String,
    pub level: i32,
    pub pos: Coordinate,
    #[serde(default)]
    pub abilities: Abilities,
}

impl EnemySpawn {
    pub fn experience(&self) -> Experience {
        Experience::Level(self.level)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct MissionData {
    pub name: String,
    pub reward: MissionReward,
    pub players: Vec<PlayerSpawn>,
    pub enemies: Vec<EnemySpawn>,
}
