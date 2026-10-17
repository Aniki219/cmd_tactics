use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::White;

use crate::{
    phases::{
        Phase, Transition,
        action_menu::{ACTION_MENU_ITEMS, ActionMenuState},
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
                blink: false,
            },
            selected,
            tiles: Vec::new(),
        }
    }

    fn confirm(&mut self, world: &mut World) -> Transition {
        {
            if world
                .at(self.cursor.pos)
                .any(|e| e.id != self.selected && matches!(e.layer, Layer::Entity))
            {
                return Transition::None;
            };
            if self.tiles.iter().any(|t| t.pos == self.cursor.pos) {
                world.move_to(self.selected, self.cursor.pos);
                return Transition::Push(Box::new(ActionMenuState::new(
                    self.selected,
                    &ACTION_MENU_ITEMS,
                )));
            };

            Transition::Pop // Invalid tile returns to cat turn
        }
    }
}

impl Phase for PlayerSelectedState {
    fn on_enter(&mut self, world: &mut World) {
        let Some(origin) = world.get(self.selected).map(|e| e.pos) else {
            return;
        };
        let Some(cat) = world.players.get(&self.selected) else {
            return;
        };
        let step = cat.data.speed;

        for i in -step..=step {
            for j in -step..=step {
                if i.abs() + j.abs() < 4 {
                    self.tiles.push(Renderable {
                        pos: origin.plus(Coordinate(i, j)),
                        glyph: String::from("* "),
                        color: White,
                        layer: BackgroundUi,
                        blink: false,
                    });
                }
            }
        }
    }

    fn on_resume(&mut self, world: &mut World) {
        let Some(to) = world.get(self.selected).map(|p| p.prev_pos) else {
            return;
        };
        world.move_to(self.selected, to);
    }

    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition {
        let mut move_cursor = |x, y| {
            self.cursor.pos = self.cursor.pos.plus(Coordinate(x, y));

            // If the cursor moves ontop of an Entity it should blink
            self.cursor.blink = world.at(self.cursor.pos).any(|_| true);
            return Transition::None;
        };

        match key.code {
            // Pressing A on a MoveTile should move you there
            // on an Entity should do nothing
            // on an empty tile should cancel the move command and return to catTurn
            KeyCode::Char('a') => self.confirm(world),
            KeyCode::Char('s') => Transition::Pop,

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => Transition::None,
        }
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        self.tiles
            .iter()
            .map(|t| t.clone())
            .chain(vec![self.cursor.clone()])
            .collect()
    }
}
