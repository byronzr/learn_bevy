///! Collider type.
/// Solid 就是默认的碰撞体类型,
/// Sensor 是传感器类型,需要追加 insert
/// Collider 相关的 Component 要通常要毗邻于 Collider(即,在同一个 Bundle 中),
/// 很多时候 Collider 会作为子级添加到 Entity 中,特别要注意这一点
use std::f32::consts::PI;

use bevy::{color::palettes::css::LIME, ecs::bundle, prelude::*};
use bevy_rapier2d::prelude::*;

// 为了方便拿到父级 entity 使用一个 自定义的 Marker 进行标记
#[derive(Component, Debug)]
struct ColliderMarker;

#[derive(Component, Debug)]
struct Ground;

#[derive(Component, Debug)]
struct Divider;

#[derive(Component, Debug)]
struct GroupTest(usize);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    //app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_systems(
        Startup,
        (setup, mass, friction, restitution, group_test).chain(),
    );
    app.add_systems(Update, show_grid);
    app.run();
}

// 碰撞组是主要去事件与触点相关
fn group_test(
    mut commands: Commands,
    mut query: Query<(&GroupTest, &mut CollisionGroups, &mut SolverGroups), With<ColliderMarker>>,
    query_un_init: Query<(Entity, &GroupTest), With<ColliderMarker>>,
) {
    // 使用 insert 方式,要注意 *Groups 要与 Collider 在同一个 Bundle 中
    for (entity, group) in query_un_init.iter() {
        if group.0 == 4 {
            commands
                .entity(entity)
                .insert(SolverGroups::new(Group::GROUP_9, Group::GROUP_9))
                .insert(CollisionGroups::new(Group::GROUP_9, Group::GROUP_9));
        } else {
            commands
                .entity(entity)
                .insert(SolverGroups::new(Group::GROUP_1, Group::GROUP_1))
                .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));
        }
    }

    // 使用 mut 方式修改,要注意,在初始时,要保证 *Groups Component 已经存在
    // for (group, mut collision_groups, mut solver_groups) in &mut query {
    //     if group.0 == 4 {
    //         collision_groups.memberships = Group::GROUP_9;
    //         collision_groups.filters = Group::GROUP_9;

    //         solver_groups.memberships = Group::GROUP_9;
    //         solver_groups.filters = Group::GROUP_9;
    //     } else {
    //         collision_groups.memberships = Group::GROUP_1;
    //         collision_groups.filters = Group::GROUP_1;

    //         solver_groups.memberships = Group::GROUP_1;
    //         solver_groups.filters = Group::GROUP_1;
    //     }
    // }
}

// 弹性
fn restitution(
    mut commands: Commands,
    query: Query<Entity, With<ColliderMarker>>,
    ground: Single<Entity, With<Ground>>,
) {
    let target = *ground;
    commands.entity(target).insert(Restitution::new(0.7));

    for (i, entity) in query.iter().enumerate() {
        commands
            .entity(entity)
            .insert(Restitution::new(i as f32 * 0.2));
    }
}

// 摩擦力
fn friction(
    ground: Single<Entity, With<Ground>>,
    mut commands: Commands,
    query: Query<Entity, With<ColliderMarker>>,
) {
    let target = *ground;
    // 摩擦力的域值为0.0 ~ 1.0 ,但也可以超过1.0
    commands.entity(target).insert(Friction::new(100.));

    for entity in query.iter() {
        commands.entity(entity).insert(Friction::new(1000.));
    }
}

// mass
fn mass(query: Query<(Entity, &ColliderMarker), With<RigidBody>>, mut commands: Commands) {
    for (i, (entity, _marker)) in query.iter().enumerate() {
        let mass = i as f32 * 10. + 5.;
        // 1. 简单设置密度
        // commands
        //     .entity(entity)
        //     .insert(ColliderMassProperties::Density(10.));
        // 2. 简单设置质量
        // commands
        //     .entity(entity)
        //     .insert(ColliderMassProperties::Mass(mass));

        // 3. 详细设置质量
        commands
            .entity(entity)
            .insert(ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(50., 50.), // 质心相对位置,影响旋转中心
                mass,                                      // 质量
                principal_inertia: 10.,                    // 主惯性(影响碰撞体旋转)
            }));
    }
}

/// 构建了两个实体,一个是传感器,一个是实体
/// setup 完成后 collider 就已经具备了重力,密度,已经能够进行自然下落
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assert_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // false = solid collider
    // true = sensor collider
    let sensor_list = [false, true, false, true, false, true];

    // 六边形
    let shape_polygon = RegularPolygon::new(15., 6);
    let shape_ploygon_handle = meshes.add(shape_polygon);

    // 球形
    let shape_ball = Circle::new(15.);
    let shape_ball_handle = meshes.add(shape_ball);

    // 颜色
    let shape_color = materials.add(Color::from(LIME));

    // shape 在保持不旋转的情况下,获得的顶点可以很好的与传感器对齐
    // 如果在这里进行旋转,那么传感器的位置就会有所偏移
    let angle = 0.;
    let vertexes: Vec<Vec2> = shape_polygon.vertices(angle).into_iter().collect();
    let half_y = 720. / 2.;
    for (i, s) in sensor_list.iter().enumerate() {
        let mut transform = Transform::from_translation(Vec3::new(i as f32 * 100. - 150., 0., 0.));
        // angle 为弧度,而不是度数,为了精度,通常以 PI 为被除数进行计算
        transform.rotate_local_z(PI / 4.);
        commands
            .spawn((
                RigidBody::Dynamic,
                Mesh2d(if i < 2 {
                    shape_ploygon_handle.clone()
                } else {
                    shape_ball_handle.clone()
                }),
                MeshMaterial2d(shape_color.clone()),
                transform,
                // 在父级进行 `交互组(collision_groups)` 与 `解算组(solver_groups)` 的设置是无用功
                // CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                // SolverGroups::new(Group::GROUP_1, Group::GROUP_1),
            ))
            .with_children(|parent| {
                // convex_hull 根据顶点进行多边行绘制与bevy原生的绘制方式有区别的地方是返回 Option
                // collider 以 children 的方式添加到 entity 中,可以很好的继承父级的相对形变,
                // 对于异形的碰撞体,这是一个很好的选择
                let collider = if i < 2 {
                    let Some(v) = Collider::convex_hull(&vertexes) else {
                        error!("Failed to create collider");
                        return;
                    };
                    v
                } else {
                    Collider::ball(15.)
                };

                // collier shape 并不会继承父级的相对路径
                let mut transform = Transform::from_xyz(0., 0., 9.);
                transform.rotate_local_z(angle);

                //
                let bundle = (
                    collider,
                    transform,
                    // 先指定一个默认组别,再 system 中才能正常修改
                    // CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                    // SolverGroups::new(Group::GROUP_1, Group::GROUP_1),
                    ColliderMarker,
                    GroupTest(i),
                );

                // 是否为 sensor
                if *s {
                    parent.spawn((bundle, Sensor));
                } else {
                    parent.spawn(bundle);
                };

                // 没有 rotate
                parent.spawn((
                    Text2d::new(format!("{}", i)),
                    TextFont {
                        font: assert_server.load("fonts/SourceHanSansCN-Normal.otf"),
                        font_size: 24.,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Transform::from_xyz(0., 0., 9.),
                ));
            });
    }

    // 添加一个隔板,
    let mut transform = Transform::from_translation(Vec3::new(0., -half_y + 155., 0.));
    // 让地面稍微倾斜
    transform.rotate_local_z(PI / 1.01);
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(1280., 30.)),
        RigidBody::Dynamic,
        Divider,
        ColliderMassProperties::Mass(0.), // 无质量,所以悬浮
        // Collider 的长宽是独立于 Sprite 的,可以从 Debug 中看出
        Collider::cuboid(800., 15.),
        CollisionGroups::new(Group::GROUP_9, Group::GROUP_9),
        SolverGroups::new(Group::GROUP_9, Group::GROUP_9),
        transform,
    ));

    // 添加地面
    let mut transform = Transform::from_translation(Vec3::new(0., -half_y + 55., 0.));
    transform.rotate_local_z(PI / 1.01);
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(1280., 30.)),
        RigidBody::Fixed,
        Ground,
        // Collider 的长宽是独立于 Sprite 的,可以从 Debug 中看出
        Collider::cuboid(800., 15.),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        SolverGroups::new(Group::GROUP_1, Group::GROUP_1),
        transform,
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
