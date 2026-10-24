use serde::Deserialize;

use crate::game_ui::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct Coordinate(pub i32, pub i32);

impl Coordinate {
    pub fn center() -> Self {
        Coordinate(GRID_WIDTH as i32 / 4 - 2, GRID_HEIGHT as i32 / 2 - 1 as i32)
    }

    pub fn plus(self, other: Coordinate) -> Coordinate {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }

    // Number of steps to get from this coord to another coord
    pub fn dist(self, other: Coordinate) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
