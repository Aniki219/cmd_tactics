use ratatui::style::Color;
use serde::Deserialize;

use crate::{
    types::{Coordinate, EntityId, EntityKind, Layer},
    world::World,
};

pub const PLAYER_COLORS: [Color; 10] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Magenta,
    Color::LightRed,
    Color::LightYellow,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightMagenta,
];

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub player: Vec<PlayerDef>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerDef {
    pub name: String,
    pub health: i32,
    pub pos: Coordinate,
}

pub fn add_player(world: &mut World, players: &mut Vec<EntityId>, pos: Coordinate) {
    players.push(world.spawn(
        EntityKind::Player(players.len() as i32 + 1),
        pos,
        Layer::Entity,
        PLAYER_COLORS[players.len() % PLAYER_COLORS.len()],
    ));
}
