use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{App, AppState},
    game::Game,
    types::{Layer::ForegroundUi, Renderable, coordinate::Coordinate, glyphs},
};

pub const GRID_WIDTH: u16 = 44;
pub const GRID_HEIGHT: u16 = 20;

const MAX_WIDTH: u16 = 60;
const HUD_HEIGHT: u16 = 4;

pub fn render(app: &App, frame: &mut Frame) {
    match &app.state {
        AppState::InGame(game) => render_game(game, frame),
        AppState::Paused(game) => render_game(game, frame),
        AppState::MainMenu() => render_menu(frame),
    }
}

pub fn render_menu(_frame: &mut Frame) {}

pub fn render_game(game: &Game, frame: &mut Frame) {
    let blink = frame.count() % 4 > 1;

    let mut renderables: HashMap<Coordinate, Vec<Renderable>> = HashMap::new();
    for r in game.phases.renderables(&game.world) {
        renderables.entry(r.pos).or_default().push(r);
    }

    let mut lines: Vec<Line> = Vec::with_capacity(GRID_HEIGHT as usize);

    for y in 0..GRID_HEIGHT {
        let mut row_spans = Vec::new();
        let mut empty_buffer = String::new();
        for x in (0..GRID_WIDTH).step_by(2) {
            let pos = Coordinate(x as i32 / 2, y as i32);

            let top_renderable = game
                .world
                .at(pos)
                .map(|e| game.world.renderable(e))
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

    let field_w = GRID_WIDTH as u16 + 2;
    let field_h = GRID_HEIGHT as u16 + 2;
    let data_w = MAX_WIDTH - GRID_WIDTH;

    let [root] = Layout::horizontal([Constraint::Max(MAX_WIDTH)])
        .flex(Flex::Center)
        .areas(frame.area());

    // Rows: field/data row, flexible gap, HUD footer
    let [_, top, hud_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(field_h),
        Constraint::Length(HUD_HEIGHT),
        Constraint::Fill(1),
    ])
    .areas(root);

    // Top row: field, data tab, leftover space
    let [field_area, menu_area, _rest] = Layout::horizontal([
        Constraint::Length(field_w),
        Constraint::Length(data_w),
        Constraint::Fill(1),
    ])
    .areas(top);

    let inner_menu = menu_area.inner(Margin::new(2, 1));
    // game.phases.render(frame, field_area, menu_area, hud_area);
    game.phases.render(frame, inner_menu);
    let Some(phase) = game.phases.current() else {
        return;
    };
    let theme = phase.theme();

    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Field ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme.field_color)),
            )
            .alignment(Alignment::Center),
        field_area,
    );

    frame.render_widget(
        Paragraph::new("").block(
            Block::default()
                .title(" Menu ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.menu_color)),
        ),
        menu_area,
    );

    frame.render_widget(
        Paragraph::new("").block(
            Block::default()
                .title(" HUD ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.hud_color)),
        ),
        hud_area,
    );
}

fn _centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
