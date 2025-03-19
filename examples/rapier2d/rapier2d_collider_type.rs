///! Collider type.
/// Solid 就是默认的碰撞体类型,
/// Sensor 是传感器类型,需要追加 insert
///
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_systems(Startup, setup);
    app.add_systems(Update, show_grid);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let colliders = [false, true];
    let half_x = 1280. / 2.;
    let half_y = 720. / 2.;
    for (i, s) in colliders.iter().enumerate() {
        let x = -half_x + 100. + i as f32 * 100.;
        let y = half_y - 100.;
        let entity = commands
            .spawn((
                Sprite::from_color(Color::WHITE, Vec2::splat(30.)),
                RigidBody::Dynamic,
                // 以邻近方式创建一个矩形碰撞体
                //Collider::cuboid(15., 15.),
                Transform::from_translation(Vec3::new(x, y, 0.)),
            ))
            .with_children(|parent| {
                parent.spawn(Collider::cuboid(15., 15.));
            })
            .id();
        if *s {
            commands.entity(entity).insert(Sensor);
        }
    }
}

// 显示网格方便观察
fn show_grid(mut gizmos: Gizmos) {
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
}
