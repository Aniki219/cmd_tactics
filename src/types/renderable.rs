use ratatui::style::Color;

use crate::{game::Game, types::*};

#[derive(Debug)]
pub enum Renderable {
    Static { glyph: char, layer: Layer },
    Entity(EntityId),
}

impl Renderable {
    pub fn layer(&self, game: &Game) -> Layer {
        match self {
            Renderable::Static { layer, .. } => *layer,
            Renderable::Entity(id) => {
                let entity = game
                    .entities
                    .get(id)
                    .expect(&format!("entity id: {} not found in grid.entities", id.0));
                entity.layer
            }
        }
    }

    pub fn glyph_and_color(&self, game: &Game) -> (String, Color) {
        match self {
            Renderable::Static { glyph, .. } => (glyph.to_string(), Color::White),
            Renderable::Entity(id) => {
                let entity = game
                    .entities
                    .get(id)
                    .expect(&format!("entity id: {} not found in grid.entities", id.0));
                match entity.kind {
                    EntityKind::Player => (glyphs::player(entity.id), entity.color),
                    EntityKind::MoveTile => (glyphs::MOVE_TILE.to_string(), entity.color),
                    EntityKind::Cursor => (glyphs::CURSOR.to_string(), entity.color),
                    _ => (glyphs::EMPTY.to_string(), Color::White),
                    
                }
            }
        }
    }
}
