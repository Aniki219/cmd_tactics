use crossterm::event::KeyEvent;
use ratatui::style::Color;

use crate::{
    phases::{Phase, Transition, enemy_turn::EnemyTurnState, player_turn::PlayerTurnState},
    types::{EntityId, EntityKind, Layer::ForegroundUi, Renderable},
    util::timer::Timer,
    world::World,
};

pub struct DealDamageState {
    source_id: EntityId,
    target_id: EntityId,
    timer: Timer,
    damage: i32,
    damage_number: Option<Renderable>,
}

impl DealDamageState {
    pub fn new(source_id: EntityId, target_id: EntityId) -> Self {
        Self {
            source_id,
            target_id,
            timer: Timer::new(4),
            damage: 0,
            damage_number: None,
        }
    }
}

impl Phase for DealDamageState {
    fn handle_input(&mut self, _key: KeyEvent, _world: &mut World) -> Transition {
        Transition::None
    }

    fn on_tick(&mut self, world: &mut World) -> Transition {
        if self.timer.is_done() {
            let Some(source_kind) = world.get(self.source_id).map(|e| e.kind) else {
                return Transition::None;
            };
            let Some(target_entity) = world.entities.get(&self.target_id) else {
                return Transition::None;
            };
            match source_kind {
                EntityKind::Cat(_) => {
                    return Transition::Reset(Box::new(PlayerTurnState::new(target_entity.pos)));
                }
                EntityKind::Enemy(_) => {
                    return Transition::Reset(Box::new(EnemyTurnState::new(world)));
                }
                _ => {
                    todo!();
                }
            }
        }
        self.timer.tick();
        Transition::None
    }

    fn on_enter(&mut self, world: &mut World) -> Transition {
        // Read phase: shared borrows only, ending once we've computed values
        let (Some(source), Some(target), Some(target_entity)) = (
            world.get_cat(&self.source_id),
            world.get_cat(&self.target_id),
            world.entities.get(&self.target_id),
        ) else {
            return Transition::None;
        };
        
        self.damage = (source.health - target.health / 2).clamp(0, 99);

        self.damage_number = Some(Renderable {
            pos: target_entity.pos,
            glyph: format!("{:02}", self.damage),
            color: Color::Yellow,
            layer: ForegroundUi,
            blink: false,
        });

        // Write phase: the shared borrows above are no longer used, so they end here
        if let Some(target) = world.cat_mut(&self.target_id) {
            target.health -= self.damage;
        }
    
        Transition::None
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        self.damage_number.clone().into_iter().collect()
    }
}
