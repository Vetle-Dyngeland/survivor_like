use super::{Player, PlayerSet};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet::Input))
            .add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

fn init(mut cmd: Commands, query: Query<Entity, With<Player>>) {
    use KeyCode::*;
    cmd.entity(query.single())
        .insert(InputManagerBundle::with_map(
            InputMap::default().with_dual_axis(
                PlayerAction::Move,
                KeyboardVirtualDPad::new(KeyW, KeyS, KeyA, KeyD),
            ),
        ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Move,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis
        }
    }
}
