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
        if self.timer.tick() {
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
        Transition::None
    }

    fn on_enter(&mut self, world: &mut World) -> Transition {
        let [Some(source_cat), Some(target_cat)] = [self.source_id, self.target_id]
            .map(|id| world.players.get(&id).or_else(|| world.enemies.get(&id)))
        else {
            return Transition::None;
        };
        self.damage = (source_cat.data.health - target_cat.data.health / 2).min(99);

        let Some(target_entity) = world.entities.get(&self.target_id) else {
            return Transition::None;
        };
        self.damage_number = Some(Renderable {
            pos: target_entity.pos,
            glyph: format!("{:02}", self.damage),
            color: Color::Yellow,
            layer: ForegroundUi,
            blink: false,
        });
        Transition::None
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        self.damage_number.clone().into_iter().collect()
    }
}
