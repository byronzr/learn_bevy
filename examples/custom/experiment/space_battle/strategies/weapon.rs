use crate::resources::turret::TurretResource;
use bevy::prelude::*;

pub fn weapons_maintenance(mut turret: ResMut<TurretResource>, time: Res<Time>) {
    for weapon in turret.weapon.iter_mut() {
        // 未挂载武器
        if weapon.entity.is_none() {
            continue;
        }
        // 增加冷却时间
        let Some(t) = weapon.shot_timer.as_mut() else {
            continue;
        };
        t.tick(time.delta());

        // 为弹匣填充
        let Some(t) = weapon.charge_timer.as_mut() else {
            continue;
        };
        if t.tick(time.delta()).just_finished() && weapon.capacity_repeat {
            if weapon.capacity < weapon.capacity_max {
                weapon.capacity += 1;
            }
        }
    }
}
