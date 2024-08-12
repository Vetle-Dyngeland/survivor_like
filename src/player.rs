use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod input;
pub mod movement;
pub mod camera;
pub mod visuals;
pub mod weapons;
pub mod health;

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugins)
            .add_systems(Startup, init.in_set(PlayerSet::Main));
    }
}

struct PlayerPlugins;
impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerSetPlugin)
            .add(input::PlayerInputPlugin)
            .add(movement::PlayerMovementPlugin)
            .add(camera::PlayerCameraPlugin)
            .add(visuals::PlayerVisualsPlugin)
            .add(weapons::PlayerWeaponPlugin)
            .add(health::PlayerHealthPlugin)
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Player; // Marker Component

fn init(mut cmd: Commands) {
    cmd.spawn((Player, Name::new("Player")));
}

struct PlayerSetPlugin;
impl Plugin for PlayerSetPlugin {
    fn build(&self, app: &mut App) {
        use PlayerSet::*;

        app.configure_sets(
            Startup,
            (
                PrePlayer, Main, Input, Movement, Damage, Camera, Visuals, PostPlayer,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                PrePlayer, Main, Input, Movement, Damage, Camera, Visuals, PostPlayer,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord, Default)]
pub enum PlayerSet {
    PrePlayer,
    #[default]
    Main,
    Input,
    Movement,
    Damage,
    Camera,
    Visuals,
    PostPlayer,
}
