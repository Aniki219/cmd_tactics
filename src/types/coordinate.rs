use crate::app::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate(pub i32, pub i32);

impl Coordinate {
    pub fn center() -> Self {
        Coordinate(GRID_WIDTH / 4 - 2, GRID_HEIGHT / 2 - 1)
    }
}
