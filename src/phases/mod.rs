use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{
    log,
    types::Renderable,
    ui::game_ui::Panels,
    ui::{game_ui::UiTheme, hud::Hud},
    world::World,
};

pub mod action_menu;
pub mod deal_damage;
pub mod end_turn;
pub mod enemy_turn;
pub mod field_menu;
pub mod init_game;
pub mod player_attack;
pub mod player_selected;
pub mod player_turn;

pub trait Phase {
    fn name(&self) -> &str {
        let full = std::any::type_name::<Self>();
        full.rsplit("::").next().unwrap_or(full)
    }
    fn on_enter(&mut self, _world: &mut World) -> Transition {
        Transition::None
    }
    fn on_resume(&mut self, _world: &mut World) -> Transition {
        Transition::None
    }
    fn on_tick(&mut self, _world: &mut World) -> Transition {
        Transition::None
    }
    fn on_exit(&mut self, _world: &mut World) {}
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> Transition {
        Transition::None
    }
    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        vec![]
    }

    fn render(&mut self, world: &World, frame: &mut Frame, panels: &mut Panels) {
        frame.render_widget(Hud::new(world), panels.hud_area());
    }

    fn theme(&self) -> UiTheme {
        UiTheme::default()
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Phase>),
    Pop,
    PopN(i32),
    Replace(Box<dyn Phase>),
    Reset(Box<dyn Phase>),
}

pub struct PhaseStack {
    pub stack: Vec<Box<dyn Phase>>,
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

    pub fn tick(&mut self, world: &mut World) {
        let Some(top) = self.stack.last_mut() else {
            return;
        };
        let transition = top.on_tick(world);
        self.apply(transition, world);
    }

    fn apply(&mut self, t: Transition, world: &mut World) {
        match t {
            Transition::None => {}
            Transition::Push(mut next) => {
                let t = next.on_enter(world);
                log!("Push Transition {}", next.name());
                self.stack.push(next);
                self.apply(t, world);
            }
            Transition::Pop => {
                let mut t = Transition::None;
                if self.stack.len() > 1 {
                    if let Some(mut old) = self.stack.pop() {
                        log!("Pop Transition {}", old.name());
                        old.on_exit(world);
                    }
                    if let Some(next) = self.stack.last_mut() {
                        log!("Resuming {}", next.name());
                        t = next.on_resume(world);
                    }
                }
                self.apply(t, world);
            }
            Transition::PopN(mut n) => {
                let mut t = Transition::None;
                while self.stack.len() > 1 && n > 0 {
                    if let Some(mut old) = self.stack.pop() {
                        log!("Pop Transition {}", old.name());
                        old.on_exit(world);
                    }
                    if let Some(next) = self.stack.last_mut() {
                        log!("Resuming {}", next.name());
                        t = next.on_resume(world);
                    }
                    n -= 1;
                }
                self.apply(t, world);
            }
            Transition::Replace(mut next) => {
                if let Some(mut old) = self.stack.pop() {
                    log!("Replacing Transition {}", old.name());
                    old.on_exit(world);
                }
                let t = next.on_enter(world);
                log!("Replacing with Transition {}", next.name());
                self.stack.push(next);
                self.apply(t, world);
            }
            Transition::Reset(mut next) => {
                while let Some(mut old) = self.stack.pop() {
                    log!("Resetting Transition {}", old.name());
                    old.on_exit(world);
                }
                let t = next.on_enter(world);
                log!("Resetting with Transition {}", next.name());
                self.stack.push(next);
                self.apply(t, world);
            }
        }
    }
}
