use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::Color,
    widgets::{Block, List, Paragraph},
};

use crate::{
    phases::{Phase, player_selected::PlayerSelectedState},
    setup::PlayerData,
    types::{Coordinate, EntityId, Layer::BackgroundUi, Renderable},
    util::helpers::ToVecString,
    world,
};

pub const ACTION_MENU_ITEMS: [&str; 3] = ["Attack", "Skill", "End"];

pub struct ActionMenuState {
    cursor_pos: Coordinate,
    player_id: EntityId,
    player: PlayerData,
    prev_pos: Coordinate,
    selected_item: i32,
    actions: Vec<String>,
}

impl ActionMenuState {
    pub fn new(
        cursor_pos: Coordinate,
        player_id: EntityId,
        player: PlayerData,
        prev_pos: Coordinate,
        actions: &[&str],
    ) -> Self {
        Self {
            cursor_pos,
            player_id,
            player,
            prev_pos,
            selected_item: 0,
            actions: actions.to_strings(),
        }
    }
}

impl Phase for ActionMenuState {
    fn handle_input(
        &mut self,
        key: crossterm::event::KeyEvent,
        world: &mut crate::world::World,
    ) -> Option<Box<dyn Phase>> {
        match key.code {
            KeyCode::Char('s') => {
                world.move_to(self.player_id, self.prev_pos);
                return Some(Box::new(PlayerSelectedState::new(
                    self.prev_pos,
                    self.player_id,
                )));
            }
            KeyCode::Up => {
                self.selected_item -= 1;
                if self.selected_item < 0 {
                    self.selected_item = self.actions.len() as i32 - 1;
                };
            }
            KeyCode::Down => {
                self.selected_item = (self.selected_item as i32 + 1) % self.actions.len() as i32;
            }
            _ => (),
        }
        None
    }

    fn renderables(&self) -> Vec<Renderable> {
        vec![Renderable {
            pos: self.prev_pos,
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
            Paragraph::new(self.player.name.clone()).block(Block::default()),
            data_layout[0],
        );

        frame.render_widget(
            List::new(self.actions.iter().map(|s| s.clone())).block(Block::default()),
            data_layout[1],
        );
    }
}
