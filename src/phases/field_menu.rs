use crossterm::event::KeyCode;
use ratatui::{Frame, style::Color};

use crate::{
    phases::{
        Phase, Transition, UiTheme,
        end_turn::{EndTurnState, Turn},
    },
    ui::game_ui::Panels,
    ui::select_list::SelectList,
    world::World,
};

pub const FIELD_MENU_ITEMS: [&str; 3] = ["Cats", "Yield", "End Turn"];

pub struct FieldMenuState {
    menu: SelectList,
}

impl FieldMenuState {
    pub fn new() -> Self {
        Self {
            menu: SelectList::new(FIELD_MENU_ITEMS.iter().map(|s| s.to_string()).collect()),
        }
    }
}

impl Phase for FieldMenuState {
    fn handle_input(&mut self, key: crossterm::event::KeyEvent, _world: &mut World) -> Transition {
        self.menu.handle_input(key);

        match key.code {
            KeyCode::Char('s') => return Transition::Pop,
            KeyCode::Char('a') => match self.menu.selected().as_str() {
                "End Turn" => {
                    return Transition::Reset(Box::new(EndTurnState::new(Turn::Player)));
                }
                _ => {}
            },
            _ => {}
        }
        Transition::None
    }

    fn render(&mut self, _world: &World, frame: &mut Frame, panels: &mut Panels) {
        frame.render_widget(&mut self.menu, panels.menu_area());
    }

    fn theme(&self) -> UiTheme {
        UiTheme {
            field_color: Color::White,
            menu_color: Color::Yellow,
            ..Default::default()
        }
    }
}
