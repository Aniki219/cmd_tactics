use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    phases::{
        Phase,
        Transition::{self},
        end_turn::{EndTurnState, Turn},
        player_turn::PlayerTurnState,
    },
    types::{Cat, Coordinate, Entity, EntityId},
    ui::{GRID_HEIGHT, GRID_WIDTH},
    world::World,
};

const DIST_WEIGHT: i32 = 10;
const STEP_WEIGHT: i32 = 1;

pub enum EnemySubPhase {
    Idle,
    Moved,
    Acted,
    // I just made this, its time to split enemy actions
    // into sub phases so we can delay between them
    // Also we should use the Timer class here
}

pub struct EnemyTurnState {
    enemy_ids: VecDeque<EntityId>,
    // phase: EnemySubPhase
    delay: i32,
}

impl EnemyTurnState {
    pub fn new(world: &World) -> Self {
        Self {
            enemy_ids: world.enemies.iter().map(|(id, _)| id.clone()).collect(),
            delay: 0,
        }
    }

    fn move_and_attack(&mut self, id: &EntityId, world: &mut World) {
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

        self.move_to_best_tile(*id, best_tile_coord, world);
        self.attack_nearest_player(*id, world);

        if let Some(cat) = world.enemies.get_mut(&id) {
            cat.acted = true;
        };
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
        // Wait until delay
        if self.delay < 2 {
            self.delay += 1;
            return Transition::None;
        }

        // If there is another enemy in the deque move and attack with it
        if let Some(enemy_id) = self.enemy_ids.pop_front() {
            self.move_and_attack(&enemy_id, world);
            self.delay = 0;

            return Transition::None;
        }

        // No more enemies to move, end turn
        return Transition::Reset(Box::new(EndTurnState::new(Turn::Enemy)));
    }

    fn on_enter(&mut self, world: &mut World) -> Transition {
        world.players.iter_mut().for_each(|(_, c)| c.acted = false);
        Transition::None
    }
}
