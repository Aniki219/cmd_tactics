use crossterm::event::KeyEvent;
use ratatui::{
    layout::HorizontalAlignment::{Left}, style::Color, text::Line, widgets::{Block, Padding, Paragraph},
};

use crate::{
    phases::{Phase, Transition, player_turn::PlayerTurnState},
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
            timer: Timer::new(6),
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
                    return Transition::Pop;
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

        self.damage = (source.attack - target.defense / 2).clamp(0, 99);

        self.damage_number = Some(Renderable {
            pos: target_entity.pos,
            glyph: format!("{:02}", self.damage),
            color: Color::Yellow,
            layer: ForegroundUi,
            blink: false,
        });

        // Write phase: the shared borrows above are no longer used, so they end here
        world
            .cat_mut(&self.target_id)
            .map(|t| t.take_damage(self.damage));

        Transition::None
    }

    fn on_exit(&mut self, world: &mut World) {
        if let Some(target) = world.get_cat(&self.target_id)
            && target.health <= 0
        {
            world.despawn(self.target_id);
        }
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        self.damage_number.clone().into_iter().collect()
    }

    fn render(
        &mut self,
        world: &World,
        frame: &mut ratatui::prelude::Frame,
        panels: &mut crate::ui::game_ui::Panels,
    ) {
        let (Some(source), Some(target)) = (
            world.get_cat(&self.source_id),
            world.get_cat(&self.target_id),
        ) else {
            return;
        };

        let mut lines = Vec::<Line>::new();

        lines.push(Line::raw(format!(
            "{} attacks {}",
            source.name, target.name
        )));

        if self.damage > 0 {
            lines.push(Line::raw(format!(
                "{} takes {} damage",
                target.name,
                self.damage
            )));
        };

        frame.render_widget(
            Paragraph::new(lines)
                .alignment(Left)
                .block(Block::new().padding(Padding::new(1, 1, 0, 0))),
            panels.hud_area(),
        );
    }
}
