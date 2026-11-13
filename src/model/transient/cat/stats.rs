use super::Cat;
use crate::{
    log,
    model::transient::{
        cat::status::StatScale,
        stat::{MAX_SPEED, MAX_STAT, MIN_DEF, StatName},
    },
};

impl Cat {
    pub fn health(&self) -> i32 {
        (self.health + self.get_stat_buff(StatName::Health)).clamp(0, MAX_STAT)
    }

    pub fn max_health(&self) -> i32 {
        (self.max_health + self.get_stat_buff(StatName::Health)).clamp(0, MAX_STAT)
    }

    pub fn attack(&self) -> i32 {
        (self.attack + self.get_stat_buff(StatName::Attack)).clamp(0, MAX_STAT)
    }

    pub fn defense(&self) -> i32 {
        (self.defense + self.get_stat_buff(StatName::Defense)).clamp(MIN_DEF, MAX_STAT)
    }

    pub fn special(&self) -> i32 {
        (self.special + self.get_stat_buff(StatName::Special)).clamp(0, MAX_STAT)
    }

    pub fn resistance(&self) -> i32 {
        (self.resistance + self.get_stat_buff(StatName::Resistance)).clamp(MIN_DEF, MAX_STAT)
    }

    pub fn speed(&self) -> i32 {
        (self.speed + self.get_stat_buff(StatName::Speed)).clamp(1, MAX_SPEED)
    }

    pub fn get_stat(&self, stat_name: StatName) -> i32 {
        match stat_name {
            StatName::Health => self.health(),
            StatName::MaxHealth => self.max_health(),
            StatName::Attack => self.attack(),
            StatName::Defense => self.defense(),
            StatName::Special => self.special(),
            StatName::Resistance => self.resistance(),
            StatName::Speed => self.speed(),
            _ => {
                log!("dont call get_stat with {}", stat_name);
                0
            }
        }
    }

    pub fn get_raw_stat(&self, stat_name: StatName) -> i32 {
        match stat_name {
            StatName::Health => self.health,
            StatName::MaxHealth => self.max_health,
            StatName::Attack => self.attack,
            StatName::Defense => self.defense,
            StatName::Special => self.special,
            StatName::Resistance => self.resistance,
            StatName::Speed => self.speed,
            _ => {
                log!("dont call get_base_stat with {}", stat_name);
                0
            }
        }
    }

    pub fn get_stat_buff(&self, stat_name: StatName) -> i32 {
        let base_stat = self.get_raw_stat(stat_name) as f32;
        let health = self.get_raw_stat(StatName::Health) as f32;
        let max_health = self.get_raw_stat(StatName::MaxHealth).max(1) as f32; // Prevents divide-by-zero

        let mut flat_total = 0.0_f32;
        let mut multiplier = 1.0_f32;

        self.status
            .iter()
            .filter_map(|s| s.stat_mod())
            .for_each(|(stat, buff, scaling)| {
                if stat != stat_name {
                    return;
                }
                let input = buff as f32;
                match scaling {
                    StatScale::Flat => flat_total += input,
                    StatScale::Percent => multiplier *= 1.0 + input / 100.0,
                    StatScale::PercentHealth => flat_total += health * input / 100.0,
                    StatScale::PercentMissingHealth => {
                        flat_total += (max_health - health) * input / 100.0
                    }
                };
            });

        let final_stat = (base_stat + flat_total) * multiplier;
        (final_stat - base_stat).round() as i32
    }
}

pub struct StatMod {}
