use crate::{ai::movement::AiMover, player::Player, stats::GlobalStats};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Clone, Debug, PartialEq, Default)]
pub struct Enemy {
    race: EnemyRace,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum EnemyRace {
    #[default]
    Normal,
    Tank,
    Speedy,
}

pub fn spawn_enemy(
    mut cmd: Commands,
    player_query: Query<Entity, With<Player>>,
    stats: Res<GlobalStats>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let max = window.single().size();

    let x_or_y: bool = rand::random();

    let get_val = |is_x: bool| -> f32 {
        let (primary_max, secondary_max) = if is_x { (max.y, max.x) } else { (max.x, max.y) };
        let val = (rand::random::<f32>() * 2f32 - 1f32) * primary_max;
        let other = if rand::random() {
            secondary_max
        } else {
            -secondary_max
        };

        return if is_x && x_or_y { val } else { other };
    };

    cmd.spawn((
        Enemy::default(),
        Name::new("Enemy"),
        AiMover {
            speed: 75f32 * (stats.enemy_speed_modifier + 100f32) / 100f32,
            target: player_query.single(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb_u8(255, 105, 105),
                custom_size: Some(Vec2::ONE * 50f32),
                ..Default::default()
            },
            transform: Transform::from_xyz(get_val(true), get_val(false), 0f32),
            ..Default::default()
        },
    ));
}
