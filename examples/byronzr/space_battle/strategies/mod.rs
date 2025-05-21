use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod projectile;
pub mod turret;
pub mod weapon;

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
                outside_clear,
                enemy::random_enemies,
                enemy::enemy_collision,
                weapon::weapons_maintenance,
            ),
        );

        app.add_observer(projectile::emit_observer);
    }
}

// 任何超出"危险范围"的实体都需要被销毁
// 如: 未命中的子弹
pub fn outside_clear(query: Query<(Entity, &Transform)>, mut commands: Commands) {
    let min = Vec2::new(-12800., -7200.);
    let max = Vec2::new(12800., 7200.);
    for (entity, transform) in query {
        // if entity outside despawn
        if transform.translation.x > max.x
            || transform.translation.x < min.x
            || transform.translation.y > max.y
            || transform.translation.y < min.y
        {
            // 给一个提示
            println!("outside clear: {:?}", transform.translation);
            commands.entity(entity).despawn();
        }
    }
}
