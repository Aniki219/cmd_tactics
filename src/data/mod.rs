use std::collections::HashMap;

use crate::data::{load::load_toml, player_cat::{PlayerCat, PlayerCatFile}, species::{Species, SpeciesFile}};

pub mod load;
pub mod mission;
pub mod player;
pub mod player_cat;
pub mod species;

#[derive(Default)]
pub struct GameData {
    species: HashMap<String, Species>,
    player_cats: HashMap<i32, PlayerCat>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let species = load_toml::<SpeciesFile>("data/species.toml")
            .map_err(|e| format!("cats.toml: {e:?}"))?
            .cats.into_iter().map(|c| (c.name.clone(), c)).collect();
        let player_cats = load_toml::<PlayerCatFile>("data/player_cats.toml")
            .map_err(|e| format!("player_cats.toml: {e:?}"))?
            .cats.into_iter().map(|c| (c.id, c)).collect();
        Ok(Self { species, player_cats })
    }

    pub fn species(&self, name: &str) -> Result<&Species, String> {
        self.species.get(name).ok_or_else(|| format!("no cat definition named '{name}'"))
    }

    pub fn player_cat(&self, id: i32) -> Result<&PlayerCat, String> {
        self.player_cats.get(&id).ok_or_else(|| format!("player cat id {id} not found"))
    }
}