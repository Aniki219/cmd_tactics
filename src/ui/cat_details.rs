use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment::Center, Rect},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::types::Cat;

pub struct CatDetails<'a> {
    cat: &'a Cat,
}

impl<'a> CatDetails<'a> {
    pub fn new(cat: &'a Cat) -> Self {
        Self { cat }
    }
}

impl Widget for CatDetails<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::<Line>::new();

        lines.push(Line::raw(self.cat.name.clone()));
        lines.push(Line::raw(
            self.cat.health.to_string() + "/" + self.cat.max_health.to_string().as_str(),
        ));

        Paragraph::new(lines)
            .alignment(Center)
            .block(Block::new().padding(Padding::new(1, 1, 0, 0)))
            .render(area, buf);
    }
}
