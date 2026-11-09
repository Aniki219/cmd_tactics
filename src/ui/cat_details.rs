use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment::Center, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
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

impl StatefulWidget for CatDetails<'_> {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut CatDetailsWidgetState) {
        let mut lines = Vec::<Line>::new();

        lines.push(Line::raw(self.cat.name.clone()).style(Style::new().fg(state.color)));
        lines.push(Line::raw(format!("LVL: {}", self.cat.level)));
        lines.push(Line::raw(
            self.cat.health.to_string() + "/" + self.cat.max_health.to_string().as_str(),
        ));

        lines.push(Line::raw(format!("ATK: {}", self.cat.attack)));
        lines.push(Line::raw(format!("DEF: {}", self.cat.defense)));
        lines.push(Line::raw(format!("SPE: {}", self.cat.special)));
        lines.push(Line::raw(format!("RES: {}", self.cat.resistance)));
        lines.push(Line::raw(format!("SPD: {}", self.cat.speed)));

        Paragraph::new(lines)
            .alignment(Center)
            .block(Block::new().padding(Padding::new(1, 1, 0, 0)))
            .render(area, buf);
    }

    type State = CatDetailsWidgetState;
}

pub struct CatDetailsWidgetState {
    pub glyph: String,
    pub color: Color,
}
