use crate::components::BaseVelocity;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 获得转角度
pub fn rotaion_to(target: Vec2, orgin_transform: &Transform) -> Option<(f32, f32)> {
    let delta = target - orgin_transform.translation.xy();
    let front = delta.normalize();

    // Y 轴测试
    let enemy_y = (orgin_transform.rotation * Vec3::Y).xy();
    let base_y = enemy_y.dot(front);
    if (base_y - 1.0).abs() < f32::EPSILON {
        return None;
    }
    // X 轴测试
    let enemy_x = (orgin_transform.rotation * Vec3::X).xy();
    let base_x = enemy_x.dot(front);
    // 旋转方向
    let clockwise = -f32::copysign(1.0, base_x);
    // 获得弧度值
    let angle = ops::acos(base_y.clamp(-1., 1.));

    Some((angle, clockwise))
}

// 获得"向前(Y轴)"的单位向量与目标距离
pub fn forward(target: Vec2, orgin_transform: &Transform) -> (Vec2, f32) {
    // 从飞船到目标的向量 (目标-飞船)
    let delta = target - orgin_transform.translation.xy();
    // 计算前进方向
    let distance = delta.length();
    // 获得旋转后的向量
    ((orgin_transform.rotation * Vec3::Y).xy(), distance)
}

// track target
pub fn forward_to(
    commands: &mut Commands,
    transform: &mut Transform,
    safe_distance: f32,
    base: &BaseVelocity,
    entity: Entity,
    target: Vec2,
    delta_time: f32,
    log: bool,
) -> bool {
    let mut flame = false;
    // 计算角度,NONE 表示无需旋转
    let rotate = rotaion_to(target, transform);
    if let Some((angle, clockwise)) = rotate {
        // 计算差值
        let rotation_value = clockwise * (base.torque * delta_time).min(angle);
        // 按差值旋转
        transform.rotate_z(rotation_value);
    }

    // 从飞船到目标的向量 (目标-飞船)
    let (forward, distance) = forward(target, transform);
    let max_step = distance - safe_distance;
    // 计算速度差值
    let velocity = (base.speed * delta_time).min(max_step);
    // 速度为负数(pulse反向)
    if velocity < f32::EPSILON {
        return flame;
    }

    //if rotate.is_some() {
    // 施加脉冲
    flame = true;
    //}

    let impluse = ExternalImpulse {
        impulse: forward * velocity,
        torque_impulse: base.torque,
    };

    let mut daming = Damping {
        linear_damping: base.braking.speed / base.speed,
        angular_damping: base.braking.torque / base.torque,
    };
    if rotate.is_some() {
        daming.linear_damping = 0.1;
    }
    if distance < safe_distance {
        daming.linear_damping = 0.1;
    }

    // 施加驱动力(脉冲)
    commands.entity(entity).insert(impluse).insert(daming);
    if log {
        println!(
            "entity: {:?}, target: {:?}, distance: {}, impulse: {:?}, daming: {:?}",
            entity, target, distance, impluse, daming,
        );
    }
    flame
}
