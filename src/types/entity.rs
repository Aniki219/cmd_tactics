use ratatui::style::Color;

use crate::types::{coordinate::Coordinate, layer::Layer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub i32);





#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub pos: Coordinate,
    pub color: Color,
    pub layer: Layer,
}

#[derive(Debug, Clone, Copy)]
pub enum EntityKind {
    Player,
    Enemy,
    Wall,
    MoveTile,
    Cursor,
}