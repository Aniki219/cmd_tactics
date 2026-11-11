use crossterm::event::KeyEvent;

use crate::{
    model::transient::world::World,
    phases::{Phase, Transition},
};

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
