use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod damage;

pub(super) struct SharedComponentsPlugin;
impl Plugin for SharedComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentsPlugins);
    }
}

struct ComponentsPlugins;
impl PluginGroup for ComponentsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(damage::DamagePlugin)
    }
}

pub mod size {
    use bevy::prelude::*;

    /// Simply holds a value for systems to pull from, does nothing on its own
    #[derive(Component, Clone, PartialEq, Debug)]
    pub struct Size(pub Vec2);
    impl Default for Size {
        fn default() -> Self {
            Self(Vec2::ONE * 50f32)
        }
    }
}
