use crossterm::event::KeyEvent;

use crate::world::World;

pub mod init_game;
pub mod player_selected;
pub mod player_turn;

pub trait Phase {
    fn on_enter(&mut self, _world: &mut World) {}
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>>;
}
