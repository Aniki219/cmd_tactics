use ratatui::style::Color;
use strum::{AsRefStr, Display};

use crate::types::{coordinate::Coordinate, glyphs, layer::Layer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub i32);

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub pos: Coordinate,
    pub color: Color,
    pub layer: Layer,
}

#[derive(Debug, Clone, Copy, Display, AsRefStr)]
pub enum EntityKind {
    Player(i32),
    Enemy,
    Wall,
}

impl Entity {
    pub fn glyph_and_color(&self) -> (String, Color) {
        let glyph = match self.kind {
            EntityKind::Player(num) => glyphs::player(num),
            _ => glyphs::EMPTY.to_string(),
        };
        (glyph, self.color)
    }
}
