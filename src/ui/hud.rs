use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction::Horizontal, HorizontalAlignment::Center, Layout, Rect},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{setup::CatState, world::World};

pub struct Hud {
    cat_states: Vec<CatState>,
}

impl Hud {
    pub fn new(world: &World) -> Self {
        let cat_states = world
            .players
            .iter()
            .filter_map(|(_id, cat)| Some(cat.data.clone()))
            .collect::<Vec<CatState>>();

        Self { cat_states }
    }
}

impl Widget for Hud {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let slots = Layout::default()
            .direction(Horizontal)
            .constraints(self.cat_states.iter().map(|_| Constraint::Min(15)))
            .split(area);

        for (cat, slot) in self.cat_states.iter().zip(slots.iter()) {
            Paragraph::new(
                cat.name.clone()
                    + "\nHP: "
                    + cat.health.to_string().as_str()
                    + "/"
                    + cat.max_health.to_string().as_str(),
            )
            .alignment(Center)
            .block(Block::new().padding(Padding::new(1, 1, 0, 0)))
            .render(*slot, buf);
        }
    }
}
