use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::AppState::MainMenu, game::Game};

#[derive(Default)]
pub struct App {
    pub should_quit: bool,
    pub state: AppState,
}

pub enum AppState {
    MainMenu(),
    InGame(Game),
    Paused(Game),
}

impl Default for AppState {
    fn default() -> Self {
        MainMenu()
    }
}

impl App {
    pub fn new() -> Self {
        let mut game = Game::new();
        game.init();
        Self {
            state: AppState::InGame(game),
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        match &mut self.state {
            AppState::InGame(game) => game.tick(),
            _ => (),
        }
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match &mut self.state {
            AppState::InGame(game) => game.handle_input(key_event),
            _ => (),
        }
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => self.quit(),
            _ => {}
        };
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
