use super::{Player, PlayerSet};
use crate::{
    shared_components::damage::{DamageType, Health, Hitbox},
    stats::GlobalStats,
};
use bevy::prelude::*;

pub(super) struct PlayerHealthPlugin;
impl Plugin for PlayerHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Damage));
    }
}

fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>, stats: Res<GlobalStats>) {
    cmd.entity(player_query.single()).insert((
        Health(stats.player_max_hp),
        Hitbox {
            damage: 5f32,
            damage_type: DamageType::General,
            custom_size: None,
            position_offset: None,
        },
    ));
}
