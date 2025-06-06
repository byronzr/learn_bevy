use crate::components::ship::{ShipHull, ShipPart};
use crate::events::{Emit, SeekEnemy};
use crate::resources::menu::MainMenu;
use crate::resources::turret::TurretResource;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 炮塔挂载点自行计算射程与角度
pub fn turret_detection(
    mut commands: Commands,
    mut turret: ResMut<TurretResource>,
    // 注意: 这里我们使到的是 GlobalTransform,因为 ShipPart 是以 Children 方式与 ShipHull 绑定的
    query: Populated<(Entity, &GlobalTransform), With<ShipPart>>,
    hull: Single<&Transform, With<ShipHull>>,
    read_rapier: ReadRapierContext,
    menu: Res<MainMenu>,
) -> Result {
    // 当前可用武器列表
    let available_weapons = turret.available_weapons();

    // hull position
    let hull_pos = hull.into_inner().translation.truncate();

    let rapier_context = read_rapier.single()?;
    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::ALL, Group::GROUP_19));

    // 开始检测
    for weapon in available_weapons {
        let Some(entity) = weapon.entity else {
            continue;
        };
        let Ok((_entity, transform)) = query.get(entity) else {
            continue;
        };
        // 第一阶段信息
        // ! 注意: 这里不能持有 weapon.phase 的引用,这是 rust 的限制,weapon 不能同时有一个可写引用与一个借用
        let phase_range = weapon.phase[0].range;
        // 炮塔挂载点
        let mount_pos = transform.translation().truncate();
        // 通过 hull_pos 与 挂载点,得到发射向量
        let mount_direction = (mount_pos - hull_pos).normalize();
        // 是否存在敌人
        if let Some((enemy_entity, projection)) =
            rapier_context.project_point(mount_pos, true, filter)
        {
            commands.trigger(SeekEnemy {
                enemy_entity: enemy_entity,
            });

            // 挂载点与目标向量
            let enemy_direction = projection.point - mount_pos;

            let distance = enemy_direction.length();

            // 计算 direction 向量与 hull_direction 向量的夹角
            let angle = mount_direction.angle_to(enemy_direction);
            if menu.log {
                println!(
                    "fire! capacity: {}, angle: {}/{} , distance: {}/{}",
                    weapon.capacity,
                    angle.abs(),
                    weapon.fire_angle,
                    distance,
                    phase_range,
                );
            }
            if angle.abs() < weapon.fire_angle && weapon.fire() && distance < phase_range {
                commands.trigger(Emit {
                    direction: mount_direction,
                    start_position: mount_pos,
                    weapon_type: weapon.weapon_type.clone(),
                });
            }
        }
    }

    Ok(())
}
