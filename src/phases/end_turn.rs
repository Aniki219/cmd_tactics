use crossterm::event::KeyEvent;

use crate::{
    phases::{Phase, Transition, enemy_turn::EnemyTurnState, player_turn::PlayerTurnState},
    types::Coordinate,
    util::timer::Timer,
    world::World,
};

pub struct EndTurnState {
    turn: Turn,
    delay: Timer,
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
            delay: Timer::new(4),
        }
    }
}

impl Phase for EndTurnState {
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> Transition {
        Transition::None
    }

    fn on_tick(&mut self, world: &mut World) -> Transition {
        if self.delay.tick() {
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

    fn on_exit(&mut self, world: &mut World) {
        world
            .players
            .iter_mut()
            .chain(world.enemies.iter_mut())
            .for_each(|(_, c)| c.acted = false)
    }
}
