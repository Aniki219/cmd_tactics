use serde::Deserialize;

use crate::model::{
    defs::species::Species,
    transient::{entity::EntityId, stat::Stat},
};

pub mod stats;
pub mod status;
pub use status::{StatScale::*, Status};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub name: String,
    pub level: i32,
    health: i32,
    max_health: i32,
    attack: i32,
    defense: i32,
    special: i32,
    resistance: i32,
    speed: i32,
    #[allow(dead_code)]
    growth_rate: i32,
    pub acted: bool,

    pub abilities: Abilities,

    pub status: Vec<Status>,
}

pub enum Experience {
    Level(i32),
    Points(i32),
}

impl Experience {
    pub fn level(self, growth_rate: &Stat) -> i32 {
        match self {
            Experience::Level(l) => l,
            Experience::Points(xp) => Cat::xp_to_level(xp, growth_rate.value_at_level(0)),
        }
    }
}

#[derive(Clone, Deserialize, Default)]
pub struct Abilities {
    pub q: Option<String>,
    pub w: Option<String>,
    pub e: Option<String>,
}

impl Cat {
    pub fn new(
        id: EntityId,
        species: &Species,
        experience: Experience,
        abilities: Abilities,
    ) -> Self {
        let level = experience.level(&species.growth_rate);
        let max_health = species.health.value_at_level(level) * 2;

        Self {
            id,
            name: species.name.clone(),
            level,
            health: max_health,
            max_health,
            attack: species.attack.value_at_level(level),
            defense: species.defense.value_at_level(level),
            special: species.special.value_at_level(level),
            resistance: species.resistance.value_at_level(level),
            speed: (species.speed.value_at_level(0) / 3).max(1) * 2,
            growth_rate: species.growth_rate.value_at_level(level),
            acted: false,
            abilities: abilities,
            //status: Vec::new(),
            status: vec![
                Status::Stun(1),
                Status::Snare(2),
                Status::Slow(3),
                Status::Silence(4),
            ],
        }
    }

    #[allow(non_snake_case)]
    pub fn xp_to_level(xp: i32, growth_rate: i32) -> i32 {
        let GROWTH_SCALE = 1.0 + growth_rate as f32 / 64.0;
        let XP_CURVE = (xp as f32).powf(0.33);

        ((1.0 + GROWTH_SCALE * XP_CURVE) as i32).clamp(1, 100)
    }

    // Take damage and report death
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).clamp(0, self.max_health);
    }
}
