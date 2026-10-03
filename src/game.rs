use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::types::*;

#[derive(Debug)]
pub struct Game {
    pub entities: HashMap<EntityId, Entity>,
    pub renderables: Vec<Renderable>,
    next_id: i32,
    pub grid: HashMap<Coordinate, Vec<Renderable>>,
    pub phase: GamePhase,
}

#[derive(Debug)]
pub enum GamePhase {
    Empty,
    PlayerTurn(PlayerTurnState),
    PlayerSelected(PlayerSelectedState),
}

#[derive(Debug)]
pub struct PlayerSelectedState {
    _selected: EntityId,
}

#[derive(Debug)]
pub struct PlayerTurnState {
    cursor: EntityId,
}

impl Game {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entities: HashMap::new(),
            renderables: Vec::new(),
            grid: HashMap::new(),
            phase: GamePhase::Empty,
        }
    }

    pub fn init(&mut self) {
        tracing::info!("Spawning players");
        self.add_entity(
            EntityKind::Player,
            Coordinate(4, 4),
            Layer::Entity,
            Color::Blue,
        );
        self.add_entity(
            EntityKind::Player,
            Coordinate(10, 10),
            Layer::Entity,
            Color::Green,
        );
        self.add_entity(
            EntityKind::Player,
            Coordinate(14, 12),
            Layer::Entity,
            Color::Red,
        );
        self.add_entity(
            EntityKind::MoveTile,
            Coordinate(13, 13),
            Layer::BackgroundUi,
            Color::White,
        );

        self.enter_player_turn();
    }

    pub fn enter_player_turn(&mut self) {
        tracing::info!("Spawning Cursor");
        let cursor = self.add_entity(
            EntityKind::Cursor,
            Coordinate::center(),
            Layer::ForegroundUi,
            Color::White,
        );

        self.phase = GamePhase::PlayerTurn(PlayerTurnState { cursor })
    }

    pub fn add_entity(
        &mut self,
        kind: EntityKind,
        pos: Coordinate,
        layer: Layer,
        color: Color,
    ) -> EntityId {
        let id = EntityId(self.get_next_id());
        let entity = Entity {
            id,
            pos,
            kind,
            color,
            layer,
        };

        self.entities.insert(entity.id, entity);
        self.grid
            .entry(pos)
            .or_default()
            .push(Renderable::Entity(id));

        entity.id
    }

    pub fn move_entity_by(&mut self, id: EntityId, by: Coordinate) {
        // Update entity position
        let Some(entity) = self.entities.get_mut(&id) else {
            return;
        };
        let from = entity.pos;
        let to = Coordinate(from.0 + by.0, from.1 + by.1);

        entity.pos = to.clone();

        // Update grid position index
        if let Some(renderables) = self.grid.get_mut(&from) {
            renderables.retain(|r| !matches!(r, Renderable::Entity(r_id) if *r_id == id));
        }
        self.grid
            .entry(to)
            .or_default()
            .push(Renderable::Entity(id));
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match &self.phase {
            GamePhase::PlayerTurn(state) => match key_event.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(digit) = c.to_digit(10) {
                        self.try_select(EntityId(digit as i32));
                    }
                }
                KeyCode::Left => self.move_entity_by(state.cursor, Coordinate(-1, 0)),
                KeyCode::Right => self.move_entity_by(state.cursor, Coordinate(1, 0)),
                KeyCode::Up => self.move_entity_by(state.cursor, Coordinate(0, -1)),
                KeyCode::Down => self.move_entity_by(state.cursor, Coordinate(0, 1)),
                _ => (),
            },
            GamePhase::PlayerSelected(_) => {}
            _ => {}
        }
    }

    fn try_select(&mut self, id: EntityId) {
        if self.entities.contains_key(&id) {
            tracing::info!("Selected player {}", id.0);
            //self.select_player(id);
        }
    }

    fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        return id;
    }
}
