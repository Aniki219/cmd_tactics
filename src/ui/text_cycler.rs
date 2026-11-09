use std::time::{Duration, SystemTime};

use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

const ADVANCE_DELAY: u64 = 500;

pub struct TextCycler {
    lines: Vec<String>,
    index: usize,
    last_cycled: SystemTime,
}

pub enum CyclerEvent {
    None,
    Advanced,
    Finished,
}

impl Default for TextCycler {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            index: 0,
            last_cycled: SystemTime::now(),
        }
    }
}

impl TextCycler {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            index: 0,
            last_cycled: SystemTime::now(),
        }
    }

    /// Returns a vector of borrowed string slices for the active lines.
    pub fn current(&self) -> Vec<&str> {
        let mut active_lines = Vec::new();

        // If the index is odd, show both the previous (even) line and current (odd) line together
        if self.index > 0 && self.index % 2 == 1 {
            if let Some(line1) = self.lines.get(self.index - 1) {
                active_lines.push(line1.as_str());
            };
            if let Some(line2) = self.lines.get(self.index) {
                active_lines.push(line2.as_str());
            };
        } else {
            // Otherwise, just show the single current even-indexed line
            if let Some(line) = self.lines.get(self.index) {
                active_lines.push(line.as_str());
            }
        }
        active_lines
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_last(&self) -> bool {
        self.index + 1 >= self.lines.len()
    }

    pub fn advance(&mut self) -> CyclerEvent {
        if self
            .last_cycled
            .elapsed()
            .map_or(false, |d| d >= Duration::from_millis(ADVANCE_DELAY))
        {
            if self.is_last() {
                CyclerEvent::Finished
            } else {
                self.index += 1;
                CyclerEvent::Advanced
            }
        } else {
            CyclerEvent::None
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let active_strings = self.current();
        if !active_strings.is_empty() {
            // Map each string slice directly to a Ratatui Line
            let lines: Vec<Line> = active_strings.into_iter().map(Line::raw).collect();
            frame.render_widget(Paragraph::new(lines), area);
        }
    }
}
