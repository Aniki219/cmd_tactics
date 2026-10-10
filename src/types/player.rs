use crate::{setup::{PlayerData}, types::EntityId};

pub struct Player {
    pub id: EntityId,
    pub data: PlayerData,
}
