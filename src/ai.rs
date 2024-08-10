use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod movement;

pub(super) struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AiPlugins);
    }
}

struct AiPlugins;
impl PluginGroup for AiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(movement::AiMovementPlugin)
    }
}
