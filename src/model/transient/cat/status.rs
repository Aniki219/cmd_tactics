use crate::model::transient::stat::StatName;

pub enum Status {
    // stat mod
    StatBuff(StatName, i32, StatScale),

    // no duration
    Burn,

    // stacks
    Mark(i32),

    // duration
    Snare(i32),
    Stun(i32),
    Silence(i32),
    Slow(i32),
}

pub enum StatScale {
    Flat,
    Percent,
    PercentHealth,
    PercentMissingHealth,
}