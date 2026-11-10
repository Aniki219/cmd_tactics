use crate::{data::{GameData, mission::MissionData}, world::{Side, World}};

pub fn spawn_mission(
    world: &mut World,
    data: &GameData,
    mission: &MissionData,
    loadout: &[i32],
) -> Result<(), String> {
    for spawn in &mission.players {
        let cat_id = *loadout.get(spawn.slot - 1)
            .ok_or_else(|| format!("mission slot {} has no assigned cat", spawn.slot))?;
        let cat = data.player_cat(cat_id)?;
        let species = data.species(&cat.name)?;
        world.spawn_cat(Side::Player, spawn.pos, species, cat.experience(), cat.abilities.clone());
    }
    for spawn in &mission.enemies {
        let species = data.species(&spawn.name)?;
        world.spawn_cat(Side::Enemy, spawn.pos, species, spawn.experience(), spawn.abilities.clone());
    }
    Ok(())
}