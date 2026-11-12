use serde::Deserialize;
use strum::Display;

/// A stat's base value, parsed from a letter grade like "A-".
#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(try_from = "String")]
pub struct Stat(i32);

impl Stat {
    const GROWTH_RATE: f32 = 3.75 / 12.0;
    const MIN_GROWTH_PER_LEVEL: f32 = 0.8;

    pub fn value_at_level(self, level: i32) -> i32 {
        let base = self.0;
        let per_level = (Self::GROWTH_RATE * base as f32).max(Self::MIN_GROWTH_PER_LEVEL);
        base + (per_level * level as f32).floor() as i32
    }
}

impl TryFrom<String> for Stat {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let invalid = || format!("invalid grade '{s}'");
        let mut chars = s.chars();

        let base = match chars.next().ok_or_else(invalid)? {
            'S' => 16,
            'A' => 13,
            'B' => 10,
            'C' => 7,
            'D' => 4,
            'F' => 2,
            _ => return Err(invalid()),
        };
        let modifier = match chars.next() {
            None => 0,
            Some('+') => 1,
            Some('-') => -1,
            Some(_) => return Err(invalid()),
        };
        if chars.next().is_some() {
            return Err(invalid());
        }
        Ok(Stat(base + modifier))
    }
}

#[derive(Display, Clone, Copy, PartialEq, Eq)]
pub enum StatName {
    MaxHealth,
    Health,
    Attack,
    Defense,
    Special,
    Resistance,
    Speed,
    Growth,
}

pub const MAX_SPEED: i32 = 20;
pub const MAX_STAT: i32 = 999;
pub const MIN_DEF: i32 = -100;