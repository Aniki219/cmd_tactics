use std::{collections::VecDeque};

use crate::{
    phases::{
        Phase, Transition::{self}, deal_damage::DealDamageState, end_turn::{EndTurnState, Turn},
    }, types::{Cat, Coordinate, Entity, EntityId, EntityKind}, ui::game_ui::{GRID_HEIGHT, GRID_WIDTH}, world::World,
};

const DIST_WEIGHT: i32 = 10;
const STEP_WEIGHT: i32 = 1;

pub enum SubPhase {
    Move,
    Action,
    // I just made this, its time to split enemy actions
    // into sub phases so we can delay between them
    // Also we should use the Timer class here
}

pub struct EnemyTurnState {
    enemy_ids: VecDeque<EntityId>,
    current: Option<(EntityId, SubPhase)>,
    delay: i32,
}

impl EnemyTurnState {
    pub fn new(world: &World) -> Self {
        Self {
            enemy_ids: world.enemies.iter().map(|(id, _)| id.clone()).collect(),
            current: None,
            delay: 0,
        }
    }

    fn move_to(&mut self, id: EntityId, world: &mut World) {
        let best_tile_coord = {
            let Some(entity) = world.entities.get(&id) else {
                return;
            };
            let Some(cat) = world.enemies.get(&id) else {
                return;
            };
            self.calc_best_tile(entity, cat, world)
        };

        let Some(best_tile_coord) = best_tile_coord else {
            return;
        };

        self.move_to_best_tile(id, best_tile_coord, world);
    }

    fn calc_best_tile(&self, entity: &Entity, cat: &Cat, world: &World) -> Option<Coordinate> {
        let player_positions: Vec<Coordinate> = world
            .players
            .iter()
            .filter_map(|(id, _)| world.get(*id).map(|e| e.pos))
            .collect();

        if player_positions.is_empty() {
            return None;
        }

        let mut best: Option<(i32, Coordinate)> = None;

        for x in 1..GRID_WIDTH as i32 {
            for y in 1..GRID_HEIGHT as i32 {
                let coord = Coordinate(x, y);

                if world.at(coord).any(|e| e.id != entity.id) {
                    continue;
                }
                let steps = coord.dist(entity.pos);
                if steps > cat.speed {
                    continue;
                }

                // Calculate distance from the current grid tile (coord) to the player,
                // not from the cat's current entity.pos.
                let nearest = player_positions
                    .iter()
                    .map(|p| p.dist(coord))
                    .min()
                    .unwrap(); // safe: checked non-empty above

                let score = DIST_WEIGHT * nearest + STEP_WEIGHT * steps;

                let candidate = (score, coord);
                let is_better = match &best {
                    None => true,
                    Some((best_score, _)) => score < *best_score,
                };
                if is_better {
                    best = Some(candidate);
                }
            }
        }

        best.map(|(_, coord)| coord)
    }

    fn move_to_best_tile(&self, id: EntityId, to: Coordinate, world: &mut World) {
        world.move_to(id, to);
    }

    fn pick_attack_target(&self, id: EntityId, world: &World) -> Option<EntityId> {
        let Some(pos) = world.get(id).map(|e| e.pos) else {
            return None;
        };

        world.get_cats_in_range(pos, Some(EntityKind::Cat(0))).iter().min_by_key(|c| c.health).map(|c| c.id)
    }
}

impl Phase for EnemyTurnState {
    fn on_tick(&mut self, world: &mut World) -> Transition {
        // Wait until delay
        if self.delay < 2 {
            self.delay += 1;
            return Transition::None;
        }
        self.delay = 0;

        let Some((id, subphase)) = self.current.take() else {
            return match self.enemy_ids.pop_front() {
                Some(id) if world.enemies.contains_key(&id) => {
                    self.current = Some((id, SubPhase::Move));
                    Transition::None
                } 
                Some(_) => Transition::None,
                None => Transition::Reset(Box::new(EndTurnState::new(Turn::Enemy)))
            };
        };
        
        match subphase {
            SubPhase::Move => {
                self.move_to(id, world);
                self.current = Some((id, SubPhase::Action));
                Transition::None
            },

            SubPhase::Action => {
                if let Some(cat) = world.cat_mut(&id) {
                    cat.acted = true;
                }
                match self.pick_attack_target(id, world) {
                    Some(target_id) => Transition::Push(Box::new(DealDamageState::new(id, target_id))),
                    _ => Transition::None,
                }
            }
        }
    }
}
