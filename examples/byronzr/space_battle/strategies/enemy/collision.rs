use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Projectile;
use crate::components::ship::EnemyHull;

use crate::resources::player::PlayerShipResource;

pub fn enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, Or<(With<Projectile>, With<EnemyHull>)>>,
    res: Res<PlayerShipResource>,
) {
    let Some(player) = res.ship_entity else {
        return;
    };
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // 如果碰撞由于玩家船体则不删除
                if e1 == &player || e2 == &player {
                    continue;
                }
                // 确保非船体以外的实体被清除
                if query.contains(*e1) {
                    commands.entity(*e1).try_despawn();
                }
                if query.contains(*e2) {
                    commands.entity(*e2).try_despawn();
                }
            }
            _ => {}
        }
    }
}
