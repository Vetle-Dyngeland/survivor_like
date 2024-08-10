use bevy::prelude::*;

pub(super) struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalStats::default());
    }
}

#[derive(Resource, Clone, Debug)]
/// All the global stats which are modified throughout the game
/// Modifier stats are percentage modifiers, all start at 0, (1 means 101%, 50 means 150%)
pub struct GlobalStats {
    pub player_max_hp: f32,
    /// HP regenerated per second
    pub player_hp_regeneration: f32,
    /// Percentage chance to steal 1 hp per killed enemy
    pub player_life_steal_chance: f32,
    pub player_attack_range: f32,
    pub player_pickup_range: f32,
    pub player_speed: f32,
    /// XP per pickup
    pub player_xp_gain: f32,
    /// Additional money per pickup
    pub player_money_gain_modifier: f32,

    pub player_damage: f32,
    pub player_damage_modifier: f32,
    pub player_attack_speed: f32,
    pub player_crit_chance: f32,
    pub player_crit_modifier: f32,

    pub enemy_damage_modifier: f32,
    pub enemy_speed_modifier: f32,
    /// Percentage modifier of material dropped per killed enemy
    pub enemy_drops_modifier: f32,
    pub enemy_spawn_rate_modifier: f32
}

impl Default for GlobalStats {
    fn default() -> Self {
        Self {
            player_max_hp: 10f32,
            player_hp_regeneration: 0f32,
            player_life_steal_chance: 0f32,
            player_attack_range: 100f32,
            player_pickup_range: 25f32,
            player_speed: 150f32,
            player_xp_gain: 1f32,
            player_money_gain_modifier: 0f32,
            player_damage: 1f32,
            player_damage_modifier: 0f32,
            player_attack_speed: 1f32,
            player_crit_chance: 0f32,
            player_crit_modifier: 100f32,

            enemy_damage_modifier: 0f32,
            enemy_speed_modifier: 0f32,
            enemy_drops_modifier: 0f32,
            enemy_spawn_rate_modifier: 0f32,
        }
    }
}
