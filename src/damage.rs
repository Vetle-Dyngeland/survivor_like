use bevy::prelude::*;

pub(super) struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {}
}

/// Hitbox, to hurt other hurtboxes
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Hitbox {
    pub damage: f32,
    pub damage_type: DamageType,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum DamageType {
    #[default]
    General,
}

impl Default for Hitbox {
    fn default() -> Self {
        Self {
            damage: 10f32,
            damage_type: DamageType::default(),
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Hurtbox {
    owner_entity: Entity,
    /// Include stats modifiers
    damage_percentage_modifier: f32,
}
