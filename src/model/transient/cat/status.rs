use strum::Display;

use crate::{
    log,
    model::transient::{
        cat::{
            Cat,
            Status::{Burn, Silence, Slow, Snare, StatBuff, Stun},
        },
        stat::StatName,
    },
};

#[derive(PartialEq, Eq, Clone, Copy, Display)]
pub enum Status {
    // stat mod
    StatBuff(StatName, i32, StatScale),

    // no duration
    Burn,
    Mark,

    // duration
    Snare(i32),
    Stun(i32),
    Silence(i32),
    Slow(i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatScale {
    Flat,
    Percent,
    PercentHealth,
    PercentMissingHealth,
}

impl Status {
    pub fn stat_mod(&self) -> Option<(StatName, i32, StatScale)> {
        match self {
            StatBuff(s, v, sc) => Some((*s, *v, *sc)),
            Slow(_) => Some((StatName::Speed, -30, StatScale::Percent)),
            Snare(_) => Some((StatName::Speed, -100, StatScale::Percent)),
            _ => None,
        }
    }

    pub fn prevents_acting(&self) -> bool {
        matches!(self, Stun(_))
    }

    pub fn prevents_ability(&self) -> bool {
        matches!(self, Stun(_) | Silence(_))
    }

    pub fn tick(&mut self) -> bool {
        match self {
            Silence(n) | Slow(n) | Snare(n) | Stun(n) => {
                *n = *n - 1;
                *n > 0
            }
            _ => true,
        }
    }
}

impl Cat {
    pub fn status_upkeep(&mut self) {
        if self.has_status(Burn) {
            self.take_damage(self.max_health() / 16);
        }
        self.status.retain_mut(Status::tick);
        self.acted = !self.can_act();

        self.status.iter().for_each(|s| log!("{}", s));
    }

    pub fn has_status(&self, status: Status) -> bool {
        self.status.iter().find(|s| **s == status).is_some()
    }

    pub fn can_act(&self) -> bool {
        !self.status.iter().any(|s| s.prevents_acting())
    }

    pub fn can_ability(&self) -> bool {
        self.status.iter().any(|s| s.prevents_ability())
    }
}
