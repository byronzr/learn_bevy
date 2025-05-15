use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;
use bevy_rapier2d::prelude::*;

mod control;
mod enemy;
mod player;

// UI 提示
#[derive(Component)]
struct Tip;

// UI 检查器
#[derive(Component)]
struct Inf;

// 虚拟炮台
#[derive(Component)]
pub struct VirtualTurret(pub bool);

#[derive(Resource, Default)]
pub struct SwitchResource {
    pub detect_test: bool,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.init_resource::<SwitchResource>();

    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(control::ControlsPlugin);
    app.add_plugins(enemy::EnemyPlugin);

    app.add_systems(Startup, (setup, show_grid));

    app.add_systems(Update, direct_test);

    app.run();
}

fn direct_test(
    // 注意: Camera 不是 Camera2D
    camera_query: Single<(&Camera, &GlobalTransform)>,
    //mut event_reader: EventReader<CursorMoved>,
    window: Single<&Window>,
    mut text_inf: Single<&mut Text, With<Inf>>,
    mut gizmos: Gizmos,
    // 注意: 两个 Single 都有 Transform 修改,需要用 Without 进行隔离
    mut player: Single<&mut Transform, (With<player::ShipHull>, Without<VirtualTurret>)>,
    virtual_turret: Single<(&mut VirtualTurret, &mut Transform)>,
    time: Res<Time>,
    switch: Res<SwitchResource>,
) {
    if !switch.detect_test {
        return;
    }

    println!("player: {:?}", virtual_turret.0.0);
    // 我们永远用Y轴作为左右判断的参考线
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

    gizmos.arrow_2d(Vec2::ZERO, world_pos, Color::srgb_u8(0, 255, 0));

    // 2. about cross
    let up = Vec3::Z;
    let side = front0.extend(0.).cross(up);

    // rotation
    player.rotation = Quat::from_mat3(&Mat3::from_cols(side, front0.extend(0.), up));

    // 3. about dot
    let (vt, mut transform) = virtual_turret.into_inner();
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

    if vt.0 {
        // 计算差值
        let speed = 0.1;
        let delta_angle = rotation_sign * (speed * time.delta_secs()).min(max_angle);
        transform.rotate_z(delta_angle);
    }

    text_inf.0 = format!(
        "
        {:?} : {} / size.Y
        x:{:.6} y:{:.6} / pos normalize
        x:{:.6} y:{:.6} / delta normalize
        x:{:.6} y:{:.6} / delta/length
        {val_y:.1} / Y dot
        {val_x:.1} / X dot
        {quadrant:?} / quadrant
        {max_angle:.2} / max_angle
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

fn setup(mut commands: Commands, mut gizmos_assets: ResMut<Assets<GizmoAsset>>) {
    //
    commands.spawn(Camera2d);

    // Tips
    commands.spawn((
        Text(format!(
            "Pause Game: Space\nSwitch Debug Render: Tab\nEngine Start: S\nDetect Test: I\nVirtual Turret: Q\n"
        )),
        Tip,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    // Infomation
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: 12.,
            ..default()
        },
        Inf,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            right: Val::Px(12.),
            ..default()
        },
    ));

    let mut gizmos = GizmoAsset::default();
    gizmos.arrow_2d(Vec2::ZERO, Vec2::new(0., 200.), Color::srgb_u8(255, 255, 0));
    commands.spawn((
        Gizmo {
            handle: gizmos_assets.add(gizmos),
            ..default()
        },
        VirtualTurret(false),
        Disabled,
    ));
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,                   // 投影模式
            UVec2::new(64, 36),                     // 单元格数量
            Vec2::new(20., 20.),                    // 单元格大小
            LinearRgba::gray(0.05).with_alpha(0.2), // 网格颜色
        )
        .outer_edges();
    commands.spawn((Gizmo {
        handle: gizom_assets.add(gizmos),
        ..default()
    },));
}
