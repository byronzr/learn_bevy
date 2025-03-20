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

/// 构建了两个实体,一个是传感器,一个是实体
fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 构键两个实体,一个是传感器,一个是实体
    let colliders = [false, true];
    let half_x = 1280. / 2.;
    let half_y = 720. / 2.;
    for (i, s) in colliders.iter().enumerate() {
        let x = -half_x + 800. + i as f32 * 100.;
        let y = half_y - 100.;
        let _entity = commands
            .spawn((
                Sprite::from_color(Color::WHITE, Vec2::splat(30.)),
                RigidBody::Dynamic,
                // 以邻近方式创建一个矩形碰撞体
                //Collider::cuboid(15., 15.),
                Transform::from_translation(Vec3::new(x, y, 0.)),
            ))
            .with_children(|parent| {
                // 两个矩形碰撞体放置在一个实体上,观察物理效果的复杂性是否会提升
                // Sensor 在这里同时添加才会有效果,尝试注释掉其中一个观察效果
                if *s {
                    parent.spawn((
                        Collider::cuboid(5., 5.),
                        Transform::from_xyz(-10., 0., 0.),
                        Sensor,
                    ));
                    parent.spawn((
                        Collider::cuboid(5., 5.),
                        Transform::from_xyz(10., 0., 0.),
                        Sensor,
                    ));
                } else {
                    parent.spawn((Collider::cuboid(5., 5.), Transform::from_xyz(-10., 0., 0.)));
                    parent.spawn((Collider::cuboid(5., 5.), Transform::from_xyz(10., 0., 0.)));
                }

                // 为了区别类型,添加一个文字标签,T = solid, S = sensor
                parent.spawn((
                    Text2d::new(if *s { "S" } else { "T" }),
                    TextFont {
                        font: assert_server.load("fonts/SourceHanSansCN-Normal.otf"),
                        font_size: 24.,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Transform::from_xyz(0., 0., 9.),
                ));
            })
            .id();
        // 父级实体添加传感器,并不会产生效果,传感器必须与碰撞体一起添加才有意义
        // if *s {
        //     // 添加传感器,观察碰撞效果
        //     commands.entity(_entity).insert(Sensor);
        // }
    }

    // 添加地面
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(1280., 30.)),
        RigidBody::Fixed,
        // Collider 的长宽是独立于 Sprite 的,可以从 Debug 中看出
        Collider::cuboid(300., 15.),
        Transform::from_translation(Vec3::new(0., -half_y + 55., 0.)),
    ));
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
