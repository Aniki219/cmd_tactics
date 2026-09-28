use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

#[derive(Debug)]
pub struct Player {
    pub id: i8,
    pub x: i32,
    pub y: i32,
    pub selected: bool,
    pub color: Color,
}

impl Player {
    pub fn handle_input(&mut self, key_event: KeyEvent) {
        if !self.selected {
        } else {
            match key_event.code {
                KeyCode::Left => self.x -= 2,
                KeyCode::Right => self.x += 2,
                KeyCode::Up => self.y -= 1,
                KeyCode::Down => self.y += 1,
                _ => {}
            };
        }
    }
}
