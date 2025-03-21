use std::f32::consts::PI;

///! Collider type.
/// Solid 就是默认的碰撞体类型,
/// Sensor 是传感器类型,需要追加 insert
///
use bevy::{color::palettes::css::LIME, prelude::*};
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
/// setup 完成后 collider 就已经具备了重力,密度,已经能够进行自然下落
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let sensor_list = [false, true];
    let shape = RegularPolygon::new(15., 6);
    let shape_color = materials.add(Color::from(LIME));

    // shape 在保持不旋转的情况下,获得的顶点可以很好的与传感器对齐
    // 如果在这里进行旋转,那么传感器的位置就会有所偏移
    let angle = 0.;
    let vertexes: Vec<Vec2> = shape.vertices(angle).into_iter().collect();
    let half_x = 1280. / 2.;
    let half_y = 720. / 2.;
    for (i, s) in sensor_list.iter().enumerate() {
        let mut transform = Transform::from_translation(Vec3::new(i as f32 * 100. - 150., 0., 0.));
        // angle 为弧度,而不是度数,为了精度,通常以 PI 为被除数进行计算
        transform.rotate_local_z(PI / 2.);
        commands
            .spawn((
                RigidBody::Dynamic,
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(shape_color.clone()),
                transform,
            ))
            .with_children(|parent| {
                // convex_hull 根据顶点进行多边行绘制与bevy原生的绘制方式有区别的地方是返回 Option
                // collider 以 children 的方式添加到 entity 中,可以很好的继承父级的相对形变,
                // 对于异形的碰撞体,这是一个很好的选择
                let Some(collider) = Collider::convex_hull(&vertexes) else {
                    error!("Failed to create collider");
                    return;
                };

                // collier shape 并不会继承父级的相对路径
                let mut transform = Transform::from_xyz(0., 0., 9.);
                transform.rotate_local_z(angle);

                if *s {
                    parent.spawn((collider, transform, Sensor));
                } else {
                    parent.spawn((collider, transform));
                }
            });
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
