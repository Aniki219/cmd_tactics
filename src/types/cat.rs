use crate::{setup::CatData, types::EntityId};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub speed: i32,
    pub acted: bool,
}

impl Cat {
    pub fn new(id: EntityId, data: CatData) -> Self {
        Self {
            id,
            name: data.name,
            health: data.health,
            max_health: data.health,
            speed: data.speed,
            acted: false,
        }
    }

    // Take damage and report death
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage).clamp(0, self.max_health);

        self.health <= 0
    }
}
