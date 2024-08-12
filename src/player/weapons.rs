use bevy::prelude::*;

pub(super) struct PlayerWeaponPlugin;
impl Plugin for PlayerWeaponPlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

// TODO:
// Add weapon slots to the player, either one holder entity per weapon or a single holder with a
// struct to determine amount 
// Add weapons, maybe just a knife to test and implement hitboxes and damaging and the like
