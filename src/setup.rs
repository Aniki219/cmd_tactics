use ratatui::style::Color;
use serde::Deserialize;

use crate::{
    types::{Cat, Coordinate, EntityKind, Grade, Layer}, world::World,
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

#[derive(Deserialize)]
pub struct CatToml {
    pub cats: Vec<CatData>,
}

#[derive(Deserialize, Clone)]
pub struct CatData {
    pub name: String,
    pub health: Grade,
    pub attack: Grade,
    pub defense: Grade,
    pub special: Grade,
    pub resistance: Grade,
    pub speed: Grade,
    pub growth_rate: String,
}

pub fn add_player_cat(world: &mut World, pos: Coordinate, data: &CatData) {
    let id = world.spawn(
        EntityKind::Cat(world.players.len() as i32 + 1),
        pos,
        Layer::Entity,
        CAT_COLORS[world.players.len() % CAT_COLORS.len()],
    );

    world.players.insert(id, Cat::new(id, data));
}

pub fn add_enemy_cat(world: &mut World, pos: Coordinate, data: &CatData) {
    let id = world.spawn(
        EntityKind::Enemy(world.enemies.len() as i32 + 1),
        pos,
        Layer::Entity,
        Color::Red,
    );

    world.enemies.insert(id, Cat::new(id, data));
}
