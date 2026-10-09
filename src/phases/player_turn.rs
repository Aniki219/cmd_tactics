use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    phases::{Phase, player_selected::PlayerSelectedState},
    types::{Layer::ForegroundUi, *},
    world::World,
};

pub struct PlayerTurnState {
    pub cursor: Renderable,
}

impl PlayerTurnState {
    pub fn new(cursor_pos: Coordinate) -> Self {
        Self {
            cursor: Renderable {
                pos: cursor_pos,
                glyph: String::from("[]"),
                color: White,
                layer: ForegroundUi,
            },
        }
    }

    fn select(&mut self, world: &World) -> Option<Box<dyn Phase>> {
        // Get the id of the Entity on the layer Entity at that position
        let target = world
            .at(self.cursor.pos)
            .find(|e| matches!(e.kind, EntityKind::Player(_)))
            .map(|e| e.id);

        // Call enter_select_player if we found one
        if let Some(id) = target {
            return Some(Box::new(PlayerSelectedState::new(self.cursor.pos, id)));
        }

        None
    }
}

impl Phase for PlayerTurnState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>> {
        let mut move_cursor = |x, y| {
            self.cursor.pos = self.cursor.pos.plus(Coordinate(x, y));
            return None;
        };

        match key.code {
            KeyCode::Char('a') => self.select(world),

            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    let id = EntityId(digit as i32);
                    if let Some(to) = world.get(id).map(|e| e.pos) {
                        self.cursor.pos = to;
                        return Some(Box::new(PlayerSelectedState::new(self.cursor.pos, id)));
                    }
                }
                None
            }

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => None,
        }
    }

    fn renderables(&self) -> Vec<Renderable> {
        vec![self.cursor.clone()]
    }
}
