use crate::{setup::CatData, types::{EntityId}};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub special: i32,
    pub resistance: i32,
    pub speed: i32,
    pub acted: bool,
}

impl Cat {
    pub fn new(id: EntityId, data: &CatData) -> Self {
        let max_health = data.health.value() * 4;
        
        Self {
            id,
            name: data.name.clone(),
            health: max_health,
            max_health,
            attack: data.attack.value(),
            defense: data.defense.value(),
            special: data.special.value(),
            resistance: data.resistance.value(),
            speed: data.speed.value(),
            acted: false,
        }
    }

    // Take damage and report death
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage).clamp(0, self.max_health);

        self.health <= 0
    }
}
