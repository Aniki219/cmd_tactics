use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, style::Color::White};

use crate::{
    game_ui::Panels, phases::{Phase, Transition, end_turn::{EndTurnState, Turn}, enemy_turn::EnemyTurnState, player_selected::PlayerSelectedState}, types::{Layer::ForegroundUi, *}, ui::hud::Hud, world::World,
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
                blink: false,
            },
        }
    }

    fn select(&mut self, world: &World) -> Transition {
        let pos = self.cursor.pos;
        world
            .at(pos)
            .map(|e| e.id)
            .find(|id| world.players.get(id).is_some_and(|p| !p.acted))
            .map_or(Transition::None, |id| {
                Transition::Push(Box::new(PlayerSelectedState::new(pos, id)))
            })
    }
}

impl Phase for PlayerTurnState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition {
        let mut move_cursor = |x, y| {
            self.cursor.pos = self.cursor.pos.plus(Coordinate(x, y));

            // If the cursor moves ontop of an Entity it should blink
            self.cursor.blink = world.at(self.cursor.pos).any(|_| true);
            return Transition::None;
        };

        match key.code {
            KeyCode::Char('a') => self.select(world),
            KeyCode::Char(' ') => Transition::Replace(Box::new(EnemyTurnState::new(&world))),

            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    let id = EntityId(digit as i32);
                    if let Some(to) = world.get(id).map(|e| e.pos) {
                        self.cursor.pos = to;
                        return Transition::Push(Box::new(PlayerSelectedState::new(
                            self.cursor.pos,
                            id,
                        )));
                    }
                }
                Transition::None
            }

            KeyCode::Left => move_cursor(-1, 0),
            KeyCode::Right => move_cursor(1, 0),
            KeyCode::Up => move_cursor(0, -1),
            KeyCode::Down => move_cursor(0, 1),
            _ => Transition::None,
        }
    }

    fn on_enter(&mut self, world: &mut World) -> Transition {
        if !world.players.iter().any(|(_, cat)| !cat.acted) {
            return Transition::Reset(Box::new(EndTurnState::new(Turn::Player)));
        }
        Transition::None
    }

    fn render(&mut self, world: &World, frame: &mut Frame, panels: &Panels) {
        frame.render_widget(Hud::new(world), panels.hud_area());
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        vec![self.cursor.clone()]
    }
}
