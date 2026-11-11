
use ratatui::style::Color;

use crate::model::{common::{Coordinate, Layer}, transient::entity::Entity};

#[derive(Clone)]
pub struct Renderable {
    pub pos: Coordinate,
    pub glyph: String,
    pub color: Color,
    pub layer: Layer,
    pub blink: bool,
}

impl From<&Entity> for Renderable {
    fn from(e: &Entity) -> Self {
        let (glyph, color) = e.glyph_and_color();
        Self {
            pos: e.pos,
            glyph,
            color,
            layer: e.layer,
            blink: false,
        }
    }
}
