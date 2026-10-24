use crossterm::event::KeyEvent;
use ratatui::{Frame, style::Color};

use crate::{
    phases::{Phase, Transition, enemy_turn::EnemyTurnState, player_turn::PlayerTurnState},
    types::Coordinate,
    ui::game_ui::{Panels, PopupState},
    ui::hud::Hud,
    util::timer::Timer,
    world::World,
};

pub struct EndTurnState {
    turn: Turn,
    pre_delay: Timer,
    delay: Timer,
    post_delay: Timer,
}

pub enum Turn {
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}

impl EndTurnState {
    pub fn new(turn: Turn) -> Self {
        Self {
            turn,
            pre_delay: Timer::new(4),
            delay: Timer::new(10),
            post_delay: Timer::new(2),
        }
    }
}

impl Phase for EndTurnState {
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> Transition {
        Transition::None
    }

    fn on_tick(&mut self, world: &mut World) -> Transition {
        if !self.pre_delay.tick() {
            return Transition::None;
        }
        if !self.delay.tick() {
            return Transition::None;
        }

        if self.post_delay.tick() {
            match self.turn {
                Turn::Player => {
                    return Transition::Reset(Box::new(EnemyTurnState::new(world)));
                }
                Turn::Enemy => {
                    let cursor_pos = world
                        .get_first_player()
                        .map(|p| p.pos)
                        .unwrap_or_else(|| Coordinate::center());
                    return Transition::Reset(Box::new(PlayerTurnState::new(cursor_pos)));
                }
                _ => {
                    let cursor_pos = world
                        .get_first_player()
                        .map(|p| p.pos)
                        .unwrap_or_else(|| Coordinate::center());
                    return Transition::Reset(Box::new(PlayerTurnState::new(cursor_pos)));
                }
            };
        }

        Transition::None
    }

    fn render(&mut self, world: &World, frame: &mut Frame, panels: &mut Panels) {
        frame.render_widget(Hud::new(world), panels.hud_area());

        if !self.pre_delay.is_done() || self.delay.is_done() {
            return;
        }

        let popup = match self.turn {
            Turn::Player => PopupState {
                title: String::from("End Turn"),
                body: String::from("Enemy Phase"),
                color: Color::Red,
            },
            Turn::Enemy => PopupState {
                title: String::from("End Turn"),
                body: String::from("Player Phase"),
                color: Color::Blue,
            },
            _ => todo!("no end turn state"),
        };

        panels.popup(frame, popup);
    }

    fn on_enter(&mut self, world: &mut World) -> Transition {
        world
            .players
            .iter_mut()
            .chain(world.enemies.iter_mut())
            .for_each(|(_, c)| c.acted = false);

        Transition::None
    }
}
