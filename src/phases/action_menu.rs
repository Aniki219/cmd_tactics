use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::{
    model::{
        common::{Coordinate, Layer::BackgroundUi},
        transient::{
            entity::{EntityId, EntityKind},
            world::World,
        },
    },
    phases::{
        Phase, Transition, UiTheme,
        end_turn::{EndTurnState, Turn},
        player_attack::PlayerAttackState,
        player_turn::PlayerTurnState,
    },
    ui::{game_ui::Panels, renderable::Renderable, select_list::SelectList},
};

pub struct ActionMenuState {
    cat_id: EntityId, // EntityId of the cat performing the action
    menu: SelectList,
}

impl ActionMenuState {
    pub fn new(cat_id: EntityId, world: &World) -> Result<Self, &'static str> {
        let Some(abilities) = world.get_cat(&cat_id).map(|c| c.abilities.clone()) else {
            return Err("Failed to create ActionMenuState");
        };

        let actions: Vec<String> = vec![
            Some(String::from("Attack")),
            abilities.q.as_ref().map(|q| format!("[Q] {}", q)),
            abilities.w.as_ref().map(|w| format!("[W] {}", w)),
            abilities.e.as_ref().map(|e| format!("[E] {}", e)),
            Some(String::from("End")),
        ]
        .into_iter()
        .flatten()
        .collect();

        Ok(Self {
            cat_id,
            menu: SelectList::new(actions),
        })
    }
}

impl Phase for ActionMenuState {
    fn handle_input(&mut self, key: crossterm::event::KeyEvent, world: &mut World) -> Transition {
        self.menu.handle_input(key);
        let Some(pos) = world.get(self.cat_id).map(|e| e.pos) else {
            return Transition::Reset(Box::new(PlayerTurnState::new(Coordinate::center())));
        };
        let Some(cat) = world.players.get_mut(&self.cat_id) else {
            return Transition::Reset(Box::new(PlayerTurnState::new(pos)));
        };

        match key.code {
            KeyCode::Char('s') => return Transition::Pop,
            KeyCode::Char('a') => match self.menu.selected().as_str() {
                "Attack" => {
                    let Some(first_target_pos) = world
                        .get_cats_in_range(pos, Some(EntityKind::Enemy(0)))
                        .iter()
                        .find_map(|cat| world.get(cat.id).map(|e| e.pos))
                    else {
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
                        return Transition::Reset(Box::new(EndTurnState::new(Turn::Player)));
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

    fn render(&mut self, world: &World, frame: &mut Frame, panels: &mut Panels) {
        let Some(cat) = world.players.get(&self.cat_id) else {
            return;
        };
        panels.menu(|area| {
            let [name_area, list_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(area);

            frame.render_widget(
                Paragraph::new(cat.name.clone()).style(Style::default().fg(Color::White)),
                name_area,
            );

            frame.render_widget(&mut self.menu, list_area);
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
