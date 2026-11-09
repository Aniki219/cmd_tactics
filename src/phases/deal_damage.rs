/*!
 *   | PlayerAttack | -> Push -> | DealDamage |
 *   | EnemyTurn | -> Push -> | DealDamage |
 *
 *   on_enter -> deal damage to target
 *   on_tick -> await delay
 *       match target kind
 *           player -> Reset -> | PlayerTurn |
 *           enemy
 *               defeated?
 *                   -> despawn defeated target
 *                   -> Replace -> | GetLoot | -> Pop -> | EnemeyTurn |
 *               else
 *                   -> Pop -> | EnemyTurn |
 */

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color::{self};

use crate::{
    phases::{Phase, Transition, drop_loot::DropLootState, player_turn::PlayerTurnState},
    types::{EntityId, EntityKind, Layer::ForegroundUi, Renderable},
    ui::{
        game_ui::UiTheme,
        text_cycler::{CyclerEvent::Finished, TextCycler},
    },
    util::timer::Timer,
    world::World,
};

pub struct DealDamageState {
    source_id: EntityId,
    target_id: EntityId,
    timer: Timer,
    damage: i32,
    damage_number: Option<Renderable>,
    text: TextCycler,
}

impl DealDamageState {
    pub fn new(source_id: EntityId, target_id: EntityId) -> Self {
        Self {
            source_id,
            target_id,
            timer: Timer::new(2),
            damage: 0,
            damage_number: None,
            text: TextCycler::default(),
        }
    }
}

impl Phase for DealDamageState {
    fn handle_input(&mut self, key: KeyEvent, world: &mut World) -> Transition {
        match key.code {
            KeyCode::Char('a') => {
                if !self.timer.is_done() {
                    return Transition::None;
                }
                if matches!(self.text.advance(), Finished) {
                    let Some(source_kind) = world.get(self.source_id).map(|e| e.kind) else {
                        return Transition::None;
                    };
                    let Some(target_entity) = world.entities.get(&self.target_id) else {
                        return Transition::None;
                    };
                    match source_kind {
                        EntityKind::Cat(_) => {
                            if let (Some(source), Some(target)) = (
                                world.get_cat(&self.source_id),
                                world.get_cat(&self.target_id),
                            ) && target.health <= 0
                            {
                                return Transition::Push(Box::new(DropLootState::new(
                                    source, target,
                                )));
                            }
                            return Transition::Reset(Box::new(PlayerTurnState::new(
                                target_entity.pos,
                            )));
                        }
                        EntityKind::Enemy(_) => {
                            return Transition::Pop;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
            }
            _ => (),
        }
        Transition::None
    }

    fn on_tick(&mut self, _world: &mut World) -> Transition {
        if self.timer.is_done() && self.text.index() == 0 {
            self.text.advance();
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

        let (Some(source), Some(target)) = (
            world.get_cat(&self.source_id),
            world.get_cat(&self.target_id),
        ) else {
            return Transition::None;
        };

        self.text = TextCycler::new(vec![
            format!("{} attacks {}", source.name, target.name),
            format!("{} takes {} damage", target.name, self.damage),
        ]);

        // Write phase: the shared borrows above are no longer used, so they end here
        world
            .cat_mut(&self.target_id)
            .map(|t| t.take_damage(self.damage));

        Transition::None
    }

    fn renderables(&self, _world: &World) -> Vec<Renderable> {
        if self.timer.is_done() {
            return self.damage_number.clone().into_iter().collect();
        }
        vec![]
    }

    fn theme(&self) -> UiTheme {
        UiTheme::focus_hud()
    }

    fn render(
        &mut self,
        _world: &World,
        frame: &mut ratatui::prelude::Frame,
        panels: &mut crate::ui::game_ui::Panels,
    ) {
        self.text.render(frame, panels.hud_area());
    }
}
