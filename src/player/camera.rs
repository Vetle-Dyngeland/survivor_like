use super::{Player, PlayerSet};
use bevy::prelude::*;

pub(super) struct PlayerCameraPlugin;
impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Camera));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    let cam = cmd.spawn(Camera2dBundle::default()).id();
    cmd.entity(player_query.single()).add_child(cam);
}
