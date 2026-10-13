use crate::{setup::CatData, types::EntityId};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub data: CatData,
    pub acted: bool,
}

impl Cat {
    pub fn new(id: EntityId, data: CatData) -> Self {
        Self {
            id,
            data,
            acted: false,
        }
    }
}
