use serde::Deserialize;

use crate::model::{common::Grade, defs::species::Species, transient::entity::EntityId};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub name: String,
    pub level: i32,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub special: i32,
    pub resistance: i32,
    pub speed: i32,
    pub growth_rate: i32,
    pub acted: bool,

    pub abilities: Abilities,
}

pub enum Experience {
    Level(i32),
    Points(i32),
}

impl Experience {
    pub fn level(self, growth_rate: &Grade) -> i32 {
        match self {
            Experience::Level(l) => l,
            Experience::Points(xp) => Cat::xp_to_level(xp, growth_rate.value(0)),
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
        let max_health = species.health.value(level) * 2;

        Self {
            id,
            name: species.name.clone(),
            level,
            health: max_health,
            max_health,
            attack: species.attack.value(level),
            defense: species.defense.value(level),
            special: species.special.value(level),
            resistance: species.resistance.value(level),
            speed: (species.speed.value(0) / 3).max(1) * 2,
            growth_rate: species.growth_rate.value(level),
            acted: false,
            abilities: abilities,
        }
    }

    pub fn xp_to_level(xp: i32, growth_rate: i32) -> i32 {
        (1.0 + (1.0 + growth_rate as f32 / 64.0) * (xp as f32).powf(0.33)) as i32
    }

    // Take damage and report death
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).clamp(0, self.max_health);
    }
}
