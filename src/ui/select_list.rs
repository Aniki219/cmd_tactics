use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}, widgets::{List, ListState, StatefulWidget, Widget}};

pub struct SelectList {
    pub items: Vec<String>,
    state: ListState,
}

impl SelectList {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            state: ListState::default().with_selected(Some(0)),
        }
    }
    pub fn handle_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.prev(),
            KeyCode::Down => self.next(),
            _ => (),
        }
    }

    fn next(&mut self) {
        let n = self.items.len();
        if n == 0 {
            return;
        };
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some((i + 1) % n));
    }

    fn prev(&mut self) {
        let n = self.items.len();
        if n == 0 {
            return;
        };
        let i = self.state.selected().unwrap_or(0);
        self.state.select(Some((i + n - 1) % n));
    }

    pub fn selected(&self) -> String {
        self.items[self.state.selected().unwrap_or_default()].clone()
    }
}

impl Widget for &mut SelectList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(
            List::new(self.items.iter().cloned())
                .style(Style::default().fg(Color::White))
                .highlight_symbol("> ")
                .highlight_style(Style::default().fg(Color::Yellow)),
            area,
            buf,
            &mut self.state,
        );
    }
}
