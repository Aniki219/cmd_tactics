use crate::{setup::PlayerData, types::EntityId};

pub struct Player {
    pub id: EntityId,
    pub data: PlayerData,
    pub acted: bool,
}

impl Player {
    pub fn new(id: EntityId, data: PlayerData) -> Self {
        Self {
            id,
            data,
            acted: false,
        }
    }
}
