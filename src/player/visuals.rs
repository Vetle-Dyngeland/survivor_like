use bevy::prelude::*;
use super::{Player, PlayerSet};

pub(super) struct PlayerVisualsPlugin;
impl Plugin for PlayerVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Visuals));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single()).insert(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb_u8(155, 155, 255),
            custom_size: Some(Vec2::new(50f32, 50f32)),
            ..Default::default()
        },
        ..Default::default()
    });
}
