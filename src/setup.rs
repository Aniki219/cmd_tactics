use ratatui::style::Color;
use serde::Deserialize;

use crate::{
    types::{Coordinate, EntityKind, Layer, Player}, world::World,
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
pub struct PlayerToml {
    pub player: Vec<PlayerData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerData {
    pub name: String,
    pub health: i32,
    pub pos: Coordinate,
}

pub fn add_player(world: &mut World, data: PlayerData) {
    let id = world.spawn(
        EntityKind::Player(world.players.len() as i32 + 1),
        data.pos,
        Layer::Entity,
        PLAYER_COLORS[world.players.len() % PLAYER_COLORS.len()],
    );

    world.players.push(Player {
        id,
        data
    });
}
