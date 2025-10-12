use crate::shader::MaterialEngineFlame;
use bevy::{prelude::*, sprite::Material2dPlugin};

pub mod enemy;
pub mod player;
pub mod projectile;
pub mod turret;
pub mod weapon;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum GameSet {
    Movement,
    Collision,
}

pub struct StrategiesPlugin;
impl Plugin for StrategiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<MaterialEngineFlame>::default());
        app.configure_sets(Update, (GameSet::Movement, GameSet::Collision).chain());
        // 创建一个刷新点,确保碰撞完成后的 Despawn 不再被查询
        app.add_systems(
            Update,
            ApplyDeferred
                .after(GameSet::Movement)
                .before(GameSet::Collision),
        );
        app.add_systems(
            Startup,
            (player::generate_player_ship, player::load_weapon_sounds).chain(),
        );

        app.add_systems(
            Update,
            (
                (
                    player::drift,
                    player::player_detection, // ! 可能放到 PostUpdate 中会好一些
                    turret::turret_detection, // ! 同上
                    outside_clear,
                    enemy::random_enemies,
                    enemy::enemy_movement,
                    enemy::enemy_locked,
                    weapon::weapons_maintenance,
                    projectile::seek_target_clean,
                )
                    .in_set(GameSet::Movement),
                (enemy::enemy_collision,).in_set(GameSet::Collision), // ! collision 会 despawn文档好像说 ApplyDeferred 会在 PostUpdate 之前执行
            ),
        );

        //app.add_systems(PostUpdate, );

        app.add_observer(projectile::emit_observer);
        app.add_observer(projectile::seek_observer);
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
