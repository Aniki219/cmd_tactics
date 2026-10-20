use crossterm::event::KeyCode;
use ratatui::{
    Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Style}, widgets::{List, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::{
    phases::{
        Phase, Transition, UiTheme, enemy_turn::EnemyTurnState, player_attack::PlayerAttackState,
        player_turn::PlayerTurnState,
    }, types::{Coordinate, EntityId, EntityKind, Layer::BackgroundUi, Renderable}, ui::Panels, util::helpers::ToVecString, world::World,
};

pub const ACTION_MENU_ITEMS: [&str; 3] = ["Attack", "Skill", "End"];

pub struct ActionMenuState {
    cat_id: EntityId, // EntityId of the cat performing the action
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
                "Attack" => {
                    let Some(first_target_pos) = [
                        Coordinate(1, 0),
                        Coordinate(-1, 0),
                        Coordinate(0, -1),
                        Coordinate(0, 1),
                    ]
                    .iter()
                    .find_map(|c| {
                        world
                            .at(pos.plus(*c))
                            .find(|e| matches!(e.kind, EntityKind::Enemy(_)))
                            .map(|e| e.pos)
                    }) else {
                        return Transition::None;
                    };
                    return Transition::Push(Box::new(PlayerAttackState::new(
                        self.cat_id,
                        pos,
                        first_target_pos,
                    )));
                }
                "End" => {
                    cat.acted = true;

                    if world.players.iter().all(|(_, cat)| cat.acted) {
                        return Transition::Reset(Box::new(EnemyTurnState::new(world)));
                    }
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
            blink: false,
        }]
    }

    fn render(&self, world: &World, frame: &mut Frame, panels: &Panels) {
        let Some(cat) = world.players.get(&self.cat_id) else { return };
        panels.menu(|area| {
            let [name_area, list_area] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .areas(area);
    
            frame.render_widget(
                Paragraph::new(cat.data.name.clone()).style(Style::default().fg(Color::White)),
                name_area,
            );
    
            frame.render_widget(
                ActionMenu {
                    items: &self.actions,
                    selected: self.selected_item,
                }, 
                list_area
            );
        });
    }

    fn theme(&self) -> UiTheme {
        UiTheme {
            field_color: Color::White,
            menu_color: Color::Yellow,
            ..Default::default()
        }
    }
}

pub struct ActionMenu<'a> {
    pub items: &'a[String],
    pub selected: usize,
}

impl Widget for ActionMenu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default().with_selected(Some(self.selected));
        StatefulWidget::render(
            List::new(self.items.iter().cloned())
                .style(Style::default().fg(Color::White))
                .highlight_symbol("> ")
                .highlight_style(Style::default().fg(Color::Yellow)),
            area,
            buf,
            &mut state,
        );
    }
}
