use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod projectile;
pub mod turret;

pub struct StrategiesPlugin;
impl Plugin for StrategiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::generate_player_ship);
        app.add_systems(
            Update,
            (
                player::drift,
                player::player_detection,
                turret::turret_detection,
                projectile::outside_clear,
                enemy::random_enemies,
                enemy::enemy_collision,
            ),
        );

        app.add_observer(projectile::emit_observer);
    }
}
