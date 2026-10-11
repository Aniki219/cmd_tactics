use crossterm::event::KeyEvent;

use crate::{phases::{Phase, Transition}, world::World};

pub struct InitGame {}

impl InitGame {
    pub fn new() -> Self {
        Self {}
    }
}

impl Phase for InitGame {
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> Transition {
        Transition::None
    }
}
