///! Point Pojection 的重要特性是: 没有指定方向,只有源点,自动寻找最近的可碰撞体,进行投射
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
struct Index(usize);

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        RapierDebugRenderPlugin::default(),
    ));

    app.add_systems(Startup, (setup, show_grid));

    app.add_systems(
        Update,
        (
            // make_new_collider_inside,
            // make_new_collider_outside,
            movement_ball,
            projection,
        )
            .chain(),
    );

    app.run();
}

fn movement_ball(mut query: Query<(&mut Transform, &Index)>, time: Res<Time>) {
    for (mut transform, index) in query.iter_mut() {
        let x = if index.0 == 0 { -300. } else { 300. };
        let y = time.elapsed_secs().sin() * 100.;

        transform.translation = Vec3::new(x, y, 0.);
    }
}

// 创建地板
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Left - Ball_0
    commands.spawn((
        Collider::ball(50.),
        Transform::from_xyz(-300., 0., 0.),
        Index(0),
    ));

    // Right - Ball_1
    commands.spawn((
        Collider::ball(50.),
        Transform::from_xyz(300., 0., 0.),
        Index(1),
    ));
}

fn projection(
    read_rapier: ReadRapierContext,
    mut gizmos: Gizmos,
    // idle 会无法读取鼠标位置
    //mut event: EventReader<CursorMoved>,
    windows: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let (camera, global_transform) = *camera;

    // 获得 App 的 window
    let window = *windows;

    // 获得鼠标的位置(不包括 title bar mac中),所以鼠标位置是基于 Canvas 的
    let Some(cursor_position) = window.cursor_position() else {
        warn_once!("cursor_position not found");
        return Ok(());
    };

    let point = camera.viewport_to_world_2d(global_transform, cursor_position)?;

    gizmos.cross_2d(point, 12., Color::srgb_u8(128, 0, 0));

    let solid = true;
    let filter = QueryFilter::default();

    let rapier_context = read_rapier.single()?;
    if let Some((entity, projection)) = rapier_context.project_point(point, solid, filter) {
        println!("entity:{:?}, projection: {:?}", entity, projection);
        gizmos.arrow_2d(point, projection.point, Color::srgb_u8(0, 128, 0));
    }
    Ok(())
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY, // 投影模式
            UVec2::new(16, 9),    // 单元格数量
            Vec2::new(80., 80.),  // 单元格大小
            // Dark gray
            LinearRgba::gray(0.05), // 网格颜色
        )
        .outer_edges();
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
