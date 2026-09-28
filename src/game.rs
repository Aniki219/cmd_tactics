use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::glyphs::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate(pub i32, pub i32);

#[derive(Debug)]
pub enum Renderable {
    Static { glyph: char, layer: Layer },
    Entity(EntityId),
}

impl Renderable {
    pub fn layer(&self, game: &Game) -> Layer {
        match self {
            Renderable::Static { layer, .. } => *layer,
            Renderable::Entity(id) => {
                let entity = game
                    .entities
                    .get(id)
                    .expect(&format!("entity id: {} not found in grid.entities", id.0));
                entity.layer
            }
        }
    }

    pub fn glyph_and_color(&self, game: &Game) -> (String, Color) {
        match self {
            Renderable::Static { glyph, .. } => (glyph.to_string(), Color::White),
            Renderable::Entity(id) => {
                let entity = game
                    .entities
                    .get(id)
                    .expect(&format!("entity id: {} not found in grid.entities", id.0));
                match entity.kind {
                    EntityKind::Player => (glyphs::player(entity.id), entity.color),
                    EntityKind::MoveTile => (glyphs::MOVE_TILE.to_string(), entity.color),
                    _ => (glyphs::EMPTY.to_string(), Color::White),
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Layer {
    Background,
    BackgroundUi,
    Entity,
    ForegroundUi,
}

#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub pos: Coordinate,
    pub color: Color,
    pub layer: Layer,
}

#[derive(Debug)]
pub enum EntityKind {
    Player,
    Enemy,
    Wall,
    MoveTile,
}

#[derive(Debug)]
pub struct Game {
    pub entities: HashMap<EntityId, Entity>,
    pub renderables: Vec<Renderable>,
    next_id: i32,
    pub grid: HashMap<Coordinate, Vec<Renderable>>,
    selected_entity_id: Option<EntityId>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            next_id: 1,
            entities: HashMap::new(),
            renderables: Vec::new(),
            grid: HashMap::new(),
            selected_entity_id: None,
        };

        tracing::info!("Spawning players");
        game.add_entity(
            EntityKind::Player,
            Coordinate(4, 4),
            Layer::Entity,
            Color::Blue,
        );
        game.add_entity(
            EntityKind::Player,
            Coordinate(10, 10),
            Layer::Entity,
            Color::Green,
        );
        game.add_entity(
            EntityKind::Player,
            Coordinate(14, 12),
            Layer::Entity,
            Color::Red,
        );
        game.add_entity(
            EntityKind::MoveTile,
            Coordinate(13, 13),
            Layer::BackgroundUi,
            Color::White,
        );

        game
    }

    pub fn add_entity(&mut self, kind: EntityKind, pos: Coordinate, layer: Layer, color: Color) {
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
    }

    pub fn move_entity(&mut self, id: EntityId, to: Coordinate) {
        // Update entity position
        let Some(entity) = self.entities.get_mut(&id) else {
            return;
        };
        let from = entity.pos;
        entity.pos = to;

        // Update grid position index
        if let Some(renderables) = self.grid.get_mut(&from) {
            renderables.retain(|r| !matches!(r, Renderable::Entity(r_id) if *r_id == id));
        }
        self.grid
            .entry(to)
            .or_default()
            .push(Renderable::Entity(id));
    }

    fn move_selected(&mut self, dx: i32, dy: i32) {
        let Some(id) = self.selected_entity_id else {
            return;
        };
        let Some(entity) = self.entities.get(&id) else {
            return;
        };
        let to = Coordinate(entity.pos.0 + dx, entity.pos.1 + dy);
        self.move_entity(id, to);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(digit) = c.to_digit(10) {
                    self.try_select(EntityId(digit as i32));
                }
            }
            KeyCode::Left => self.move_selected(-1, 0),
            KeyCode::Right => self.move_selected(1, 0),
            KeyCode::Up => self.move_selected(0, -1),
            KeyCode::Down => self.move_selected(0, 1),
            _ => (),
        }
    }

    fn try_select(&mut self, id: EntityId) {
        if self.entities.contains_key(&id) {
            tracing::info!("Selected player {}", id.0);
            self.select_player(id);
        }
    }

    fn select_player(&mut self, id: EntityId) {
        self.selected_entity_id = Some(id);
    }

    fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        return id;
    }
}
