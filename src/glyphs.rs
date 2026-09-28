use std::char;

use crate::game::EntityId;

pub fn player(id: EntityId) -> String {
    let code_point = 0x2460 + ((id.0 - 1) as u32);
    // let code_point = 0x2776 + ((*i - 1) as u32);

    char::from_u32(code_point).map(|c| c.to_string()).unwrap() + " "
}

pub const EMPTY: &str = "  ";
pub const MOUNTAIN: &str = "/\\";
pub const MOVE_TILE: &str = "\u{25A2}";
