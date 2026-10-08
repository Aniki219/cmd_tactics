use serde::Deserialize;

use crate::app::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct Coordinate(pub i32, pub i32);

impl Coordinate {
    pub fn center() -> Self {
        Coordinate(GRID_WIDTH / 4 - 2, GRID_HEIGHT / 2 - 1)
    }

    pub fn plus(self, other: Coordinate) -> Coordinate {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }
}
