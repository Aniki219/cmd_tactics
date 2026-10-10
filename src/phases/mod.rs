use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::{types::Renderable, world::World};

pub mod init_game;
pub mod player_selected;
pub mod player_turn;
pub mod action_menu;

pub trait Phase {
    fn on_enter(&mut self, _world: &mut World) {}
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>>;
    fn renderables(&self) -> Vec<Renderable> {vec![]}
    fn render_menu(&self, _frame: &mut Frame, _area: Rect) {}
}
