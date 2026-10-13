use ratatui::style::Color;
use serde::Deserialize;

use crate::{
    types::{Cat, Coordinate, EntityKind, Layer},
    world::World,
};

pub const CAT_COLORS: [Color; 10] = [
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Magenta,
    Color::LightYellow,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightRed,
    Color::Red,
];

#[derive(Debug, Deserialize)]
pub struct CatToml {
    pub cat: Vec<CatData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CatData {
    pub name: String,
    pub health: i32,
    pub pos: Coordinate,
}

pub fn add_cat(world: &mut World, data: CatData) {
    let id = world.spawn(
        EntityKind::Cat(world.cats.len() as i32 + 1),
        data.pos,
        Layer::Entity,
        CAT_COLORS[world.cats.len() % CAT_COLORS.len()],
    );

    world.cats.insert(id, Cat::new(id, data));
}
