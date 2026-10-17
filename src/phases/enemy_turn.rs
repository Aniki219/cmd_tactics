use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    phases::{
        Phase,
        Transition::{self},
        player_turn::PlayerTurnState,
    },
    types::{Cat, Coordinate, Entity, EntityId},
    ui::{GRID_HEIGHT, GRID_WIDTH},
    world::World,
};

const DIST_WEIGHT: i32 = 10;
const STEP_WEIGHT: i32 = 1;

pub struct EnemyTurnState {
    queue: VecDeque<EntityId>,
    delay: i32,
}

impl EnemyTurnState {
    pub fn new(world: &World) -> Self {
        Self {
            queue: world.enemies.iter().map(|(id, _)| id.clone()).collect(),
            delay: 0,
        }
    }

    fn move_and_attack(&mut self, id: &EntityId, world: &mut World) -> Transition {
        let Some(entity) = world.entities.get(&id) else {
            return Transition::None;
        };
        let Some(cat) = world.enemies.get(&id) else {
            return Transition::None;
        };
        let Some(best_tile_coord) = self.calc_best_tile(entity, cat, world) else {
            return Transition::None;
        };

        self.move_to_best_tile(*id, best_tile_coord, world);
        self.attack_nearest_player(*id, world);

        Transition::Reset(Box::new(PlayerTurnState::new(
            world
                .players
                .iter()
                .min_by_key(|(_, p)| p.id)
                .and_then(|(_, p)| world.entities.get(&p.id))
                .map_or(Coordinate::center(), |c| c.pos),
        )))
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
                if steps > cat.data.speed {
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

    fn attack_nearest_player(&self, id: EntityId, world: &World) {
        let Some(_cat) = world.enemies.get(&id) else {
            return;
        };
    }
}

impl Phase for EnemyTurnState {
    fn handle_input(&mut self, key: KeyEvent, _world: &mut World) -> Transition {
        match key.code {
            KeyCode::Char(' ') => {
                Transition::Replace(Box::new(PlayerTurnState::new(Coordinate::center())))
            }
            _ => Transition::None,
        }
    }

    fn on_tick(&mut self, world: &mut World) -> Transition {
        if self.delay < 2 {
            self.delay += 1;
            return Transition::None;
        }

        let Some(enemy_id) = self.queue.pop_front() else {
            return Transition::Reset(Box::new(PlayerTurnState::new(Coordinate::center())));
        };
        self.move_and_attack(&enemy_id, world);
        self.delay = 0;

        Transition::None
    }
}
