use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{App, AppState, GRID_HEIGHT, GRID_WIDTH},
    game::Game,
    types::{EntityKind::Player, Layer::ForegroundUi, Renderable, coordinate::Coordinate, glyphs},
};

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
    for r in game.phase.renderables() {
        renderables.entry(r.pos).or_default().push(r);
    }

    let mut lines: Vec<Line> = Vec::with_capacity(GRID_HEIGHT as usize);

    for y in 0..GRID_HEIGHT {
        let mut row_spans = Vec::new();
        let mut empty_buffer = String::new();
        for x in (0..GRID_WIDTH).step_by(2) {
            let pos = Coordinate(x / 2, y);

            let should_blink = blink && game.world.at(pos).any(|e| matches!(e.kind, Player(_)));
            let top_renderable = game
                .world
                .at(pos)
                .map(Renderable::from)
                .chain(renderables.get(&pos).into_iter().flatten().cloned())
                .filter(|e| !(should_blink && matches!(e.layer, ForegroundUi)))
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

    let content_area = centered_rect(GRID_WIDTH as u16 + 2, GRID_HEIGHT as u16 + 2, frame.area());

    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Counter App ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        content_area,
    )
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
