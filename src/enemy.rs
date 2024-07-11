use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Enemy {
    race: EnemyRace,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EnemyRace {
    Normal,
    Tank
}
impl EnemyRace {
    pub fn get_stats(self) -> EnemyStats {

    }
}

struct EnemyStats {

}
