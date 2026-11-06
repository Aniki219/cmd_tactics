use crate::{setup::CatData, types::EntityId};

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
}

pub struct CatStats {
    pub level: Option<i32>,
    pub xp: Option<i32>,
}

impl Cat {
    pub fn new(id: EntityId, base_stats: &CatData, cat_stats: &CatStats) -> Self {
        let level = cat_stats.level.unwrap_or_else(|| {
            let xp = cat_stats.xp.expect("no level or xp");
            Cat::xp_to_level(xp, base_stats.growth_rate.value(0))
        });

        let max_health = base_stats.health.value(level) * 2;

        Self {
            id,
            name: base_stats.name.clone(),
            level,
            health: max_health,
            max_health,
            attack: base_stats.attack.value(level),
            defense: base_stats.defense.value(level),
            special: base_stats.special.value(level),
            resistance: base_stats.resistance.value(level),
            speed: (base_stats.speed.value(0) / 3).max(1) * 2,
            growth_rate: base_stats.growth_rate.value(level),
            acted: false,
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
