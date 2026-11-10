use serde::Deserialize;

use crate::types::{Abilities, Experience};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct PlayerCat {
    pub id: i32,
    pub name: String,
    pub xp: i32,
    #[serde(default)]
    pub abilities: Abilities
}

impl PlayerCat {
    pub fn experience(&self) -> Experience { Experience::Points(self.xp) }
}

#[derive(Deserialize)]
pub(super) struct PlayerCatFile { pub cats: Vec<PlayerCat> }
