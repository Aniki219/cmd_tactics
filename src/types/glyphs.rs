use std::char;

pub fn player(num: i32) -> String {
    let code_point = 0x2460 + ((num - 1) as u32);

    char::from_u32(code_point).map(|c| c.to_string()).unwrap() + " "
}

pub const EMPTY: &str = "  ";
pub const MOUNTAIN: &str = "/\\";
pub const LITTLE_TREE: &str = " t";
pub const TREE: &str = "T ";
pub const DENSE_TREE: &str = "Tt";
pub const CURSOR: &str = "[]";
pub const MOVE_TILE: &str = "* ";
