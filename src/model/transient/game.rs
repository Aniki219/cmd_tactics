use std::collections::HashMap;

use crate::model::{
    defs::{
        load::load_defs,
        species::{Species, SpeciesFile},
    },
    persistent::{
        load::load_save,
        player_cat::{PlayerCat, PlayerCatFile},
    },
};

#[derive(Default)]
pub struct GameData {
    species: HashMap<String, Species>,
    player_cats: HashMap<i32, PlayerCat>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let species = load_defs::<SpeciesFile>("species.toml")
            .map_err(|e| format!("cats.toml: {e:?}"))?
            .cats
            .into_iter()
            .map(|c| (c.name.clone(), c))
            .collect();
        let player_cats = load_save::<PlayerCatFile>("player_cats.toml") //Option<Cat>
            .map_err(|e| format!("player_cats.toml: {e:?}"))?
            .ok_or_else(|| "player_cats.toml missing".to_string())?
            .cats
            .into_iter()
            .map(|c| (c.id, c))
            .collect();
        Ok(Self {
            species,
            player_cats,
        })
    }

    pub fn species(&self, name: &str) -> Result<&Species, String> {
        self.species
            .get(name)
            .ok_or_else(|| format!("no cat definition named '{name}'"))
    }

    pub fn player_cat(&self, id: i32) -> Result<&PlayerCat, String> {
        self.player_cats
            .get(&id)
            .ok_or_else(|| format!("player cat id {id} not found"))
    }
}
