use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod ai;
pub mod enemy;
pub mod player;
pub mod stats;
pub mod shared_components;

struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(player::PlayerPlugin)
            .add(stats::StatsPlugin)
            .add(ai::AiPlugin)
            .add(shared_components::SharedComponentsPlugin)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        title: "Survivors like".to_string(),
                        focused: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(GamePlugins)
        .add_systems(
            Startup,
            (
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
                enemy::spawn_enemies,
            )
                .after(player::PlayerSet::Main),
        )
        .insert_resource(ClearColor(Color::srgb_u8(25, 25, 25)))
        .run();
}
