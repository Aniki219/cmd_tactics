use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    phases::{
        Phase,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
        player_turn::PlayerTurnState,
    },
    types::{
        Layer::{BackgroundUi, ForegroundUi},
        *,
    },
    world::World,
};

pub struct PlayerSelectedState {
    cursor: Renderable,
    selected: EntityId,
    prev_pos: Coordinate,
    tiles: Vec<Renderable>,
}

impl PlayerSelectedState {
    pub fn new(cursor_pos: Coordinate, selected: EntityId) -> Self {
        Self {
            cursor: Renderable {
                pos: cursor_pos,
                glyph: String::from("[]"),
                color: White,
                layer: ForegroundUi,
            },
            prev_pos: cursor_pos,
            selected,
            tiles: Vec::new(),
        }
    }

    fn back(&self) -> Box<dyn Phase> {
        Box::new(PlayerTurnState::new(self.cursor.pos))
    }

    fn confirm(&mut self, world: &mut World) -> Option<Box<dyn Phase>> {
        {
            if world
                .at(self.cursor.pos)
                .any(|e| matches!(e.layer, Layer::Entity))
            {
                return None;
            };
            if self.tiles.iter().any(|t| t.pos == self.cursor.pos) {
                world.move_to(self.selected, self.cursor.pos);
                if let Some(data) = world.players.iter().find(|p| p.id == self.selected).map(|p| p.data.clone()) {
                    return Some(Box::new(ActionMenuState::new(
                        self.cursor.pos,
                        self.selected,
                        data,
                        self.prev_pos,
                        &ACTION_MENU_ITEMS,
                    )));
                }
            };

            Some(self.back())
        }
    }
}

impl Phase for PlayerSelectedState {
    fn on_enter(&mut self, world: &mut World) {
        let Some(origin) = world.get(self.selected).map(|e| e.pos) else {
            return;
        };
        for i in -3..=3_i32 {
            for j in -3..=3_i32 {
                if i.abs() + j.abs() < 4 && !(i == 0 && j == 0) {
                    self.tiles.push(Renderable {
                        pos: origin.plus(Coordinate(i, j)),
                        glyph: String::from("* "),
                        color: White,
                        layer: BackgroundUi,
                    });
                }
            }
        }
    }

    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Option<Box<dyn Phase>> {
        let mut move_cursor = |x, y| {
            self.cursor.pos = self.cursor.pos.plus(Coordinate(x, y));
            return None;
        };

        match key.code {
            // Pressing A on a MoveTile should move you there
            // on an Entity should do nothing
            // on an empty tile should cancel the move command and return to PlayerTurn
            KeyCode::Char('a') => self.confirm(world),
            KeyCode::Char('s') => Some(self.back()),

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => None,
        }
    }

    fn renderables(&self) -> Vec<Renderable> {
        self.tiles
            .iter()
            .map(|t| t.clone())
            .chain(vec![self.cursor.clone()])
            .collect()
    }
}
