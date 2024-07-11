use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod player;
pub mod stats;
pub mod enemy;

struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(player::PlayerPlugin)
            .add(stats::StatsPlugin)
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
        .insert_resource(ClearColor(Color::srgb_u8(25, 25, 25)))
        .run();
}
