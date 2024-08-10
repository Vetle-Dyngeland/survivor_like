use bevy::prelude::*;

pub(super) struct AiMovementPlugin;
impl Plugin for AiMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mover);
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct AiMover {
    pub speed: f32,
    pub target: Entity,
}

fn mover(
    mut query: Query<(&AiMover, &mut Transform)>,
    target_query: Query<&Transform, Without<AiMover>>,
    time: Res<Time>,
) {
    for (mover, mut transform) in query.iter_mut() {
        let delta = match target_query.get(mover.target) {
            Ok(t) => {
                (t.translation - transform.translation)
                    .truncate()
                    .normalize_or_zero()
                    .extend(0f32)
                    * mover.speed
                    * time.delta_seconds()
            }
            _ => {
                eprintln!("Could not find target {}", mover.target);
                continue
            },
        };
        transform.translation += delta;
    }
}
