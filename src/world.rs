use std::collections::HashMap;

use ratatui::style::Color;

use crate::{
    log,
    types::{Cat, Coordinate, Entity, EntityId, EntityKind, Layer, Renderable},
};

pub struct World {
    pub entities: HashMap<EntityId, Entity>,
    pub players: HashMap<EntityId, Cat>,
    pub enemies: HashMap<EntityId, Cat>,
    next_id: i32,
    pub grid: HashMap<Coordinate, Vec<Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
            entities: HashMap::new(),
            players: HashMap::new(),
            enemies: HashMap::new(),
            next_id: 1,
        }
    }
    pub fn spawn(
        &mut self,
        kind: EntityKind,
        pos: Coordinate,
        layer: Layer,
        color: Color,
    ) -> EntityId {
        let id = EntityId(self.get_next_id());
        let entity = Entity::new(id, kind, pos, color, layer);

        self.entities.insert(entity.id, entity);
        self.grid.entry(pos).or_default().push(entity);

        log!("Spawned {} id {}", kind, id);
        entity.id
    }

    pub fn despawn(&mut self, id: EntityId) {
        let entity_pos = self.entities.get(&id).map(|e| e.pos);

        if let Some(pos) = entity_pos {
            if let Some(cell) = self.grid.get_mut(&pos) {
                cell.retain(|e| e.id != id);
            }
        }

        self.entities.remove(&id);
    }

    pub fn move_by(&mut self, id: EntityId, by: Coordinate) {
        // Update entity position
        let Some(entity) = self.entities.get_mut(&id) else {
            return;
        };
        let from = entity.pos;
        let to = Coordinate(from.0 + by.0, from.1 + by.1);

        self.move_to(id, to);
    }

    pub fn move_to(&mut self, id: EntityId, to: Coordinate) {
        let Some(entity) = self.entities.get_mut(&id) else {
            return;
        };
        let from = entity.pos;
        entity.prev_pos = from;
        entity.pos = to.clone();

        // Update grid position index
        if let Some(entities) = self.grid.get_mut(&from) {
            entities.retain(|e| e.id != id);
        }
        self.grid.entry(to).or_default().push(*entity);
    }

    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn at(&self, pos: Coordinate) -> impl Iterator<Item = &Entity> {
        self.grid
            .get(&pos)
            .into_iter()
            .flatten()
            .filter_map(|e| self.entities.get(&e.id))
    }

    fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        return id;
    }

    pub fn renderable(&self, e: &Entity) -> Renderable {
        let mut r = Renderable::from(e);
        if self
            .players
            .get(&e.id)
            .or_else(|| self.enemies.get(&e.id))
            .is_some_and(|p| p.acted)
        {
            r.color = Color::Rgb(150, 150, 150);
        }
        r
    }

    pub fn get_first_player(&self) -> Option<&Entity> {
        self.players
            .iter()
            .min_by_key(|(_, p)| p.id)
            .and_then(|(_, p)| self.entities.get(&p.id))
    }

    pub fn cat_mut(&mut self, id: &EntityId) -> Option<&mut Cat> {
        self.players
            .get_mut(id)
            .or_else(|| self.enemies.get_mut(id))
    }

    pub fn get_cat(&self, id: &EntityId) -> Option<&Cat> {
        self.players.get(id).or_else(|| self.enemies.get(id))
    }
}
