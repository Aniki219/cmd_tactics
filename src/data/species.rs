use serde::Deserialize;

use crate::types::Grade;

#[derive(Deserialize, Clone)]
pub struct Species {
    pub name: String,
    pub health: Grade,
    pub attack: Grade,
    pub defense: Grade,
    pub special: Grade,
    pub resistance: Grade,
    pub speed: Grade,
    pub growth_rate: Grade,
}

#[derive(Deserialize)]
pub(super) struct SpeciesFile {
    pub cats: Vec<Species>,
}