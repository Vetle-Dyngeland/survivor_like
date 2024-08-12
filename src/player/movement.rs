use crate::player::{input::PlayerAction, Player, PlayerSet};
use crate::stats::GlobalStats;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Movement))
            .add_systems(Update, player_controller.in_set(PlayerSet::Movement));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single())
        .insert(SpatialBundle::default())
        .insert(PlayerController::default());
}

#[derive(Component, Clone, Debug, PartialEq, Default)]
pub struct PlayerController {
    pub custom_speed: Option<f32>,
}

fn player_controller(
    mut query: Query<(
        &mut Transform,
        &ActionState<PlayerAction>,
        &PlayerController,
    )>,
    stats: Res<GlobalStats>,
    time: Res<Time>,
) {
    for (mut transform, input, controller) in query.iter_mut() {
        transform.translation += Vec3::from((
            input.clamped_axis_pair(&PlayerAction::Move).normalize_or_zero(),
            0f32,
        )) * time.delta_seconds()
            * if let Some(speed) = controller.custom_speed {
                speed
            } else {
                stats.player_speed
            };
    }
}
