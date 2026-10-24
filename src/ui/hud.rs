use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction::Horizontal, HorizontalAlignment::Center, Layout, Rect},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{types::Cat, world::World};

pub struct Hud<'a> {
    cats: Vec<&'a Cat>,
}

impl<'a> Hud<'a> {
    pub fn new(world: &'a World) -> Self {
        Self {
            cats: world.players.values().collect(),
        }
    }
}

impl Widget for Hud<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let slots = Layout::default()
            .direction(Horizontal)
            .constraints(self.cats.iter().map(|_| Constraint::Min(15)))
            .split(area);

        for (cat, slot) in self.cats.iter().zip(slots.iter()) {
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
