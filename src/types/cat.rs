use crate::{setup::CatState, types::EntityId};

pub struct Cat {
    pub id: EntityId, // The associated Entity id
    // cat num lives in the entity kind on the entity
    pub data: CatState,
    pub acted: bool,
}

impl Cat {
    pub fn new(id: EntityId, data: CatState) -> Self {
        Self {
            id,
            data,
            acted: false,
        }
    }
}
