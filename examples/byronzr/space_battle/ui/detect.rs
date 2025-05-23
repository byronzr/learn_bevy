use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;

use crate::components::ship::ShipHull;
use crate::resources::menu::MainMenu;

// 虚拟炮台
#[derive(Component)]
pub struct VirtualTurret;

#[derive(Component, Debug)]
pub struct UILayoutDetect;

#[derive(Component, Debug)]
pub struct DebugRenderMaker;

pub fn direct_test(
    mut commands: Commands,
    // 注意: Camera 不是 Camera2D
    camera_query: Single<(&Camera, &GlobalTransform)>,
    // 注意: mut event_reader: EventReader<CursorMoved>,虽然性能更高,但是鼠标不移动时,不需产生事件,差值算法将被跳过
    window: Single<&Window>,
    mut text_inf: Single<&mut Text, With<UILayoutDetect>>,
    mut gizmos: Gizmos,
    // 注意: 两个 Single 都有 Transform 修改,需要用 Without 进行隔离
    mut player: Single<&mut Transform, (With<ShipHull>, Without<VirtualTurret>)>,
    // 注意: virtual turret 是一个 gizmos 绘制的箭头,Visiblilty似乎对它无效
    virtual_turret: Single<(Entity, &mut Transform, Option<&Disabled>), With<VirtualTurret>>,
    time: Res<Time>,
    menu: Res<MainMenu>,
) {
    let (entity, mut transform, _) = virtual_turret.into_inner();

    if !menu.detect_test {
        commands.entity(entity).insert_if_new(Disabled);
        return;
    }
    // 注意: Disabled 需要在 query 中强制添加
    commands.entity(entity).remove::<Disabled>();

    // 我们永远用Y轴作为左右判断的参考线 (白色定位指针)
    // assuming player face Y axis
    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::new(0., 200.),
        Color::srgb_u8(255, 255, 255),
    );

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };

    // 1. about normalize
    // assuming world_pos is enemy
    let delta = world_pos.extend(0.) - player.translation;
    let front0 = world_pos.normalize();
    let front1 = delta.normalize();
    let front2 = delta / delta.length();

    // 跟随鼠标方向指针(绿色)
    gizmos.arrow_2d(Vec2::ZERO, world_pos, Color::srgb_u8(0, 255, 0));

    // 2. about cross
    let up = Vec3::Z;
    let side = front0.extend(0.).cross(up);
    // rotation
    player.rotation = Quat::from_mat3(&Mat3::from_cols(side, front0.extend(0.), up));

    // 3. about dot

    // 将四元数旋转,应用到单位向量 Y 上,
    // 这样就得到一个旋转的 Y 轴,
    let base_y = (transform.rotation * Vec3::Y).xy();
    let val_y = base_y.dot(front0);

    // 如果方向一致,炮塔不需要转动了
    // 当 val_y == 0.9999..8 - 1.0
    // val_y == -0.00000..2,因为是负值可能永远小于 EPSILON ,所以我们要报绝对值
    if (val_y - 1.0).abs() < f32::EPSILON {
        return;
    }
    let base_x = Vec3::X.truncate();
    let val_x = base_x.dot(front0);

    // 有了两次点积,我们就可以知道炮台的象限了
    let quadrant = if val_y > 0. {
        if val_x > 0. { 1 } else { 2 }
    } else {
        if val_x > 0. { 4 } else { 3 }
    };

    // clockwise
    // 在右手坐标系中(Bevy),
    // Z轴顺时针旋转为负数
    // 当我们基于 X 轴确认象限时,23象限正好是负值,14象限正好是正值,正好与我们的参考 Y 轴相反
    let rotation_sign = -f32::copysign(1.0, val_x);

    // 获得一个最小的夹角
    let max_angle = ops::acos(val_y.clamp(-1., 1.));

    if menu.virtual_turret {
        // 计算差值
        let speed = 0.1;
        let delta_angle = rotation_sign * (speed * time.delta_secs()).min(max_angle);
        transform.rotate_z(delta_angle);
    }

    text_inf.0 = format!(
        "
        {:+.6} : {} / size.Y
        x:{:+.6} y:{:+.6} / pos normalize
        x:{:+.6} y:{:+.6} / delta normalize
        x:{:+.6} y:{:+.6} / delta/length
        {val_y:+.1} / Y dot
        {val_x:+.1} / X dot
        {quadrant:?} / quadrant
        {max_angle:+.2} / max_angle
        ",
        side.y,
        if side.y > 0. { "left" } else { "right" },
        front0.x,
        front0.y,
        front1.x,
        front1.y,
        front2.x,
        front2.y,
    );
}
