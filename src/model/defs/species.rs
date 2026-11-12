use serde::Deserialize;

use crate::model::transient::stat::Stat;

#[derive(Deserialize, Clone)]
pub struct Species {
    pub name: String,
    pub health: Stat,
    pub attack: Stat,
    pub defense: Stat,
    pub special: Stat,
    pub resistance: Stat,
    pub speed: Stat,
    pub growth_rate: Stat,
}

#[derive(Deserialize)]
pub struct SpeciesFile {
    pub cats: Vec<Species>,
}
