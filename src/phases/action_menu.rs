use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph},
};

use crate::{
    phases::{
        Phase, Transition, UiTheme, player_turn::PlayerTurnState,
    },
    types::{Coordinate, EntityId, Layer::BackgroundUi, Renderable},
    util::helpers::ToVecString,
    world::World,
};

pub const ACTION_MENU_ITEMS: [&str; 3] = ["Attack", "Skill", "End"];

pub struct ActionMenuState {
    cat_id: EntityId,
    selected_item: usize,
    actions: Vec<String>,
}

impl ActionMenuState {
    pub fn new(cat_id: EntityId, actions: &[&str]) -> Self {
        Self {
            cat_id,
            selected_item: 0,
            actions: actions.to_strings(),
        }
    }
}

impl Phase for ActionMenuState {
    fn handle_input(&mut self, key: crossterm::event::KeyEvent, world: &mut World) -> Transition {
        let n = self.actions.len();
        let Some(pos) = world.get(self.cat_id).map(|e| e.pos) else {
            return Transition::Reset(Box::new(PlayerTurnState::new(Coordinate::center())));
        };
        let Some(cat) = world.players.get_mut(&self.cat_id) else {
            return Transition::Reset(Box::new(PlayerTurnState::new(pos)));
        };

        match key.code {
            KeyCode::Up => self.selected_item = (self.selected_item + n - 1) % n,
            KeyCode::Down => self.selected_item = (self.selected_item + 1) % n,
            KeyCode::Char('s') => return Transition::Pop,
            KeyCode::Char('a') => match self.actions[self.selected_item].as_str() {
                "End" => {
                    cat.acted = true;
                    return Transition::Reset(Box::new(PlayerTurnState::new(pos)));
                }
                _ => {}
            },
            _ => {}
        }
        Transition::None
    }

    fn renderables(&self, world: &World) -> Vec<Renderable> {
        let Some(cat) = world.get(self.cat_id) else {
            return Vec::<Renderable>::new();
        };
        vec![Renderable {
            pos: cat.prev_pos,
            glyph: "X ".to_string(),
            color: Color::Red,
            layer: BackgroundUi,
        }]
    }

    fn render_menu(&self, frame: &mut Frame, area: Rect) {
        let data_layout = Layout::default()
            .direction(Vertical)
            .constraints([
                Constraint::Length(3), // Space for the paragraph
                Constraint::Min(1),    // Remaining space for the list
            ])
            .split(area);

        frame.render_widget(
            Paragraph::new("self.cat.name.clone()")
                .block(Block::default())
                .style(Style::default().fg(Color::White)),
            data_layout[0],
        );

        let mut state = ListState::default().with_selected(Some(self.selected_item));
        frame.render_stateful_widget(
            List::new(self.actions.iter().map(|s| s.clone()))
                .block(Block::default())
                .style(Style::default().fg(Color::White))
                .highlight_symbol("> ")
                .highlight_style(Style::default().fg(Color::Yellow)),
            data_layout[1],
            &mut state,
        );
    }

    fn theme(&self) -> UiTheme {
        UiTheme {
            field_color: Color::White,
            menu_color: Color::Yellow,
            ..Default::default()
        }
    }
}
