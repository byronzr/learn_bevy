use bevy::prelude::*;

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
