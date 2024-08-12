use bevy::prelude::*;

pub use boxes::*;
pub use health::*;

pub(super) struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, death);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum DamageType {
    #[default]
    General,
}

pub mod health {
    use bevy::prelude::*;

    #[derive(Component, Clone, Debug, PartialEq)]
    pub struct Health(pub f32);

    impl Default for Health {
        fn default() -> Self {
            // NOTE: Magic number
            return Self(10f32);
        }
    }

    impl Health {
        pub fn damage(&mut self, amount: f32) {
            self.0 -= amount;
            // TODO: add death handling and visual effects
        }

        pub fn heal(&mut self, amount: f32) {
            self.0 += amount;
            // TODO: add visual effects
        }
    }

    pub(super) fn death(mut cmd: Commands, health_query: Query<(Entity, &Health)>) {
        for (entity, health) in health_query.iter() {
            if health.0 < 1f32 {
                cmd.entity(entity).despawn_recursive();
            }
        }
    }
}

pub mod boxes {
    use crate::shared_components::size::Size;

    use super::{DamageType, Health};
    use bevy::prelude::*;

    /// Hitbox, to hurt other hurtboxes
    #[derive(Component, Clone, Debug, PartialEq)]
    pub struct Hitbox {
        pub damage: f32,
        pub damage_type: DamageType,
        pub custom_size: Option<Vec2>,
        pub position_offset: Option<Vec2>,
    }

    #[derive(Component, Clone, Debug, PartialEq)]
    pub struct Hurtbox {
        /// If set to None, picks the entity itself
        pub owner: Option<Entity>,
        /// Include stats modifiers when defining
        pub custom_damage_modifier: f32,
        pub custom_size: Option<Vec2>,
        pub position_offset: Option<Vec2>,
    }
}
