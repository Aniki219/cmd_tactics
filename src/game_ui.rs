use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{
    phases::Phase,
    types::{Layer::ForegroundUi, Renderable, coordinate::Coordinate, glyphs},
    world::World,
};

pub const GRID_WIDTH: u16 = 54;
pub const GRID_HEIGHT: u16 = 20;

const MAX_WIDTH: u16 = 70;
const HUD_HEIGHT: u16 = 4;

pub struct GameUi {}

pub struct Popup {
    pub title: String,
    pub body: String,
    pub color: Color,
}

pub struct Panels {
    field: Rect,
    menu: Rect,
    hud: Rect,
}

impl Panels {
    pub fn field(&self) -> Rect {
        self.field
    }

    pub fn menu(&self, draw: impl FnOnce(Rect)) {
        draw(self.menu.inner(Margin::new(2, 1)));
    }

    pub fn hud(&self, draw: impl FnOnce(Rect)) {
        draw(self.hud.inner(Margin::new(2, 1)));
    }

    pub fn hud_area(&self) -> Rect {
        self.hud.inner(Margin::new(1, 1))
    }

    pub fn popup(&mut self, frame: &mut Frame, popup: Popup) {
        let area = centered_rect(self.field.width / 2, 3, self.field);
        let popup = Paragraph::new(popup.body)
            .wrap(Wrap { trim: true })
            .style(popup.color)
            .alignment(Alignment::Right)
            .block(
                Block::new()
                    .title(popup.title)
                    .title_style(Style::new().white().bold())
                    .borders(Borders::ALL)
                    .border_style(popup.color),
            );
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }
}

impl GameUi {
    fn panel(title: &'static str, color: Color) -> Block<'static> {
        Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color))
    }
    fn layout(frame: &Frame) -> Panels {
        let field_w = GRID_WIDTH as u16 + 2;
        let field_h = GRID_HEIGHT as u16 + 2;
        let data_w = MAX_WIDTH - GRID_WIDTH;

        let [root] = Layout::horizontal([Constraint::Max(MAX_WIDTH)])
            .flex(Flex::Center)
            .areas(frame.area());

        // Rows: field/data row, flexible gap, HUD footer
        let [_, top, hud, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(field_h),
            Constraint::Length(HUD_HEIGHT),
            Constraint::Fill(1),
        ])
        .areas(root);

        // Top row: field, data tab, leftover space
        let [field, menu, _rest] = Layout::horizontal([
            Constraint::Length(field_w),
            Constraint::Length(data_w),
            Constraint::Fill(1),
        ])
        .areas(top);

        Panels { field, menu, hud }
    }
    pub fn render(&mut self, phase: &mut dyn Phase, world: &World, frame: &mut Frame) {
        let panels = Self::layout(frame);
        let theme = phase.theme();

        let lines = Self::field_lines(phase, world, frame.count() % 4 > 1);
        frame.render_widget(
            Paragraph::new(lines)
                .block(Self::panel(" Field ", theme.field_color))
                .alignment(Alignment::Center),
            panels.field,
        );

        frame.render_widget(
            Paragraph::new("").block(Self::panel(" Menu ", theme.menu_color)),
            panels.menu,
        );

        frame.render_widget(
            Paragraph::new("").block(Self::panel(" HUD ", theme.hud_color)),
            panels.hud,
        );

        phase.render(&world, frame, &panels);
    }
    fn field_lines(phase: &dyn Phase, world: &World, blink: bool) -> Vec<Line<'static>> {
        let mut renderables: HashMap<Coordinate, Vec<Renderable>> = HashMap::new();
        for r in phase.renderables(&world) {
            renderables.entry(r.pos).or_default().push(r);
        }

        let mut lines: Vec<Line> = Vec::with_capacity(GRID_HEIGHT as usize);

        for y in 0..GRID_HEIGHT {
            let mut row_spans = Vec::new();
            let mut empty_buffer = String::new();
            for x in (0..GRID_WIDTH).step_by(2) {
                let pos = Coordinate(x as i32 / 2, y as i32);

                let top_renderable = world
                    .at(pos)
                    .map(|e| world.renderable(e))
                    .chain(renderables.get(&pos).into_iter().flatten().cloned())
                    .filter(|r| !(r.blink && blink && matches!(r.layer, ForegroundUi)))
                    .max_by_key(|e| e.layer);
                if let Some(renderable) = top_renderable {
                    if !empty_buffer.is_empty() {
                        row_spans.push(Span::raw(empty_buffer));
                        empty_buffer = String::new();
                    }
                    let style = Style::default().fg(renderable.color);
                    row_spans.push(Span::styled(renderable.glyph, style));
                } else {
                    empty_buffer.push_str(glyphs::EMPTY);
                }
            }

            if !empty_buffer.is_empty() {
                row_spans.push(Span::raw(empty_buffer));
            }

            lines.push(Line::from(row_spans));
        }
        lines
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
