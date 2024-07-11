use super::{Player, PlayerSet};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Input));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    let mut input_map = InputMap::default();
    input_map.insert(PlayerAction::Move, VirtualDPad::wasd());
    cmd.entity(player_query.single()).insert(InputManagerBundle::with_map(input_map));
}

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Move,
}
