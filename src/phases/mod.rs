use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect, style::Color};

use crate::{log, types::Renderable, world::World};

pub mod action_menu;
pub mod init_game;
pub mod player_selected;
pub mod player_turn;

pub trait Phase {
    fn name(&self) -> &str {
        let full = std::any::type_name::<Self>();
        full.rsplit("::").next().unwrap_or(full)
    }
    fn on_enter(&mut self, _world: &mut World) {}
    fn on_resume(&mut self, _world: &mut World) {}
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition;
    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        vec![]
    }
    fn render_menu(&self, _frame: &mut Frame, _area: Rect) {}
    fn theme(&self) -> UiTheme {
        UiTheme::default()
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Phase>),
    Pop,
    Replace(Box<dyn Phase>),
    Reset(Box<dyn Phase>),
}

pub struct PhaseStack {
    pub stack: Vec<Box<dyn Phase>>,
}

pub struct UiTheme {
    pub field_color: Color,
    pub menu_color: Color,
    pub hud_color: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            field_color: Color::Yellow,
            menu_color: Color::White,
            hud_color: Color::White,
        }
    }
}

impl PhaseStack {
    pub fn new(mut root: Box<dyn Phase>, world: &mut World) -> Self {
        root.on_enter(world);
        Self { stack: vec![root] }
    }

    pub fn current(&self) -> Option<&dyn Phase> {
        self.stack.last().map(|b| b.as_ref())
    }

    pub fn reset(&mut self, next: Box<dyn Phase>, world: &mut World) {
        self.apply(Transition::Reset(next), world);
    }

    pub fn handle_input(&mut self, key: KeyEvent, world: &mut World) {
        let Some(top) = self.stack.last_mut() else {
            return;
        };
        let transition = top.handle_input(key, world);
        self.apply(transition, world);
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // for phase in &self.stack {
        //     phase.render_menu(frame, area);
        // }
        if let Some(top) = self.stack.last() {
            top.render_menu(frame, area);
        };
    }

    pub fn renderables(&self, world: &World) -> Vec<Renderable> {
        // self.stack
        //     .iter()
        //     .flat_map(|phase| phase.renderables(world))
        //     .collect()
        let Some(top) = self.stack.last() else {
            return Vec::new();
        };
        top.renderables(world)
    }

    fn apply(&mut self, t: Transition, world: &mut World) {
        match t {
            Transition::None => {}
            Transition::Push(mut next) => {
                next.on_enter(world);
                log!("Push Transition {}", next.name());
                self.stack.push(next);
            }
            Transition::Pop => {
                if self.stack.len() > 1 {
                    if let Some(mut old) = self.stack.pop() {
                        log!("Pop Transition {}", old.name());
                        old.on_exit(world);
                    }
                    if let Some(next) = self.stack.last_mut() {
                        log!("Resuming {}", next.name());
                        next.on_resume(world);
                    }
                }
            }
            Transition::Replace(mut next) => {
                if let Some(mut old) = self.stack.pop() {
                    log!("Replacing Transition {}", old.name());
                    old.on_exit(world);
                }
                next.on_enter(world);
                log!("Replacing with Transition {}", next.name());
                self.stack.push(next);
            }
            Transition::Reset(mut next) => {
                while let Some(mut old) = self.stack.pop() {
                    log!("Resetting Transition {}", old.name());
                    old.on_exit(world);
                }
                next.on_enter(world);
                log!("Resetting with Transition {}", next.name());
                self.stack.push(next);
            }
        }
    }
}
