use crate::{
    ai::movement::AiMover,
    player::Player,
    shared_components::{
        damage::{DamageType, Hitbox, Hurtbox},
        size::Size,
    },
    stats::GlobalStats,
};
use bevy::{prelude::*, window::PrimaryWindow};

// Temporary function
pub fn spawn_enemies(
    mut cmd: Commands,
    player_query: Query<Entity, With<Player>>,
    stats: Res<GlobalStats>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    const AMOUNT: u16 = 10; // Magic number
    let player = player_query.single();
    let window = window.single();

    for _ in 0..AMOUNT {
        Enemy::spawn(
            &mut cmd,
            &stats,
            player,
            EnemyRace::Normal,
            Enemy::generate_random_spawn_pos(&window),
        );
    }
}

#[derive(Component, Clone, Debug, PartialEq, Default)]
pub struct Enemy {
    race: EnemyRace,
}

impl Enemy {
    fn generate_random_spawn_pos(window: &Window) -> Vec2 {
        let bounds = window.size();
        let x_or_y: bool = rand::random();

        let get_val = |is_x: bool| -> f32 {
            let (val_bounds, other_bounds) = if is_x {
                (bounds.min_element(), bounds.max_element())
            } else {
                (bounds.max_element(), bounds.min_element())
            };
            let val = (rand::random::<f32>() - 0.5f32) * 2f32 * val_bounds;
            let other = (rand::random::<bool>() as i8 as f32 * 2f32 - 1f32) * other_bounds;

            let x = is_x && x_or_y;
            return val * x as i8 as f32 + other * !x as i8 as f32;
        };

        return Vec2::new(get_val(true), get_val(false));
    }

    fn spawn(
        cmd: &mut Commands,
        stats: &Res<GlobalStats>,
        player_entity: Entity,
        race: EnemyRace,
        position: Vec2,
    ) -> Entity {
        let size = Size(Vec2::splat(50f32));

        cmd.spawn((
            Self { race },
            Name::new("Enemy"),
            AiMover {
                speed: race.get_base_speed() * (stats.enemy_speed_modifier + 100f32) / 100f32,
                target: player_entity,
            },
            Size(race.get_base_size()),
            Hurtbox {
                owner: None,
                custom_damage_modifier: 0f32,
                custom_size: None,
                position_offset: None,
            },
            Hitbox {
                damage: race.get_base_damage() * (stats.enemy_damage_modifier + 100f32) / 100f32,
                custom_size: None,
                damage_type: race.get_damage_type(),
                position_offset: None,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(255, 105, 105),
                    custom_size: Some(size.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position.x, position.y, 0f32),
                ..Default::default()
            },
        ))
        .id()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum EnemyRace {
    #[default]
    Normal,
    Tank,
    Speedy,
}

impl EnemyRace {
    pub fn get_base_damage(&self) -> f32 {
        match self {
            Self::Tank => 3f32,
            _ => 1f32,
        }
    }

    pub fn get_damage_type(&self) -> DamageType {
        match self {
            _ => DamageType::General,
        }
    }

    pub fn get_base_speed(&self) -> f32 {
        match self {
            Self::Tank => 33f32,
            Self::Speedy => 125f32,
            _ => 75f32,
        }
    }

    pub fn get_base_size(&self) -> Vec2 {
        match self {
            Self::Tank => Vec2::new(50f32, 100f32),
            _ => Vec2::splat(50f32),
        }
    }
}
