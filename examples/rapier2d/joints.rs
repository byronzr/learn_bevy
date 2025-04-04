#![allow(dead_code)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const START_X: f32 = 1280.0 / 2.0;
const START_Y: f32 = 720.0 / 2.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_systems(Startup, setup);

    app.add_systems(PostUpdate, show_grid);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 两个锚点都以 (0.0) 为中心
    // local_anchor1 通常是当前实体
    // local_anchor2 通常是父级实体
    // joint 只是建立了关系,并不会有实体的"线"存在,所以它可以穿过任何实体
    // DOF: Degree of Freedom 自由度的概念,在 Joint 中就是限制 DOF
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(30.0, 0.0))
        .local_anchor2(Vec2::new(-30.0, 0.0));
    //.motor_position(0., 0., 0.);

    // anchor1 与 anchor2 两端绑定的任意实体,都会受到 joint 的影响
    // 也会间接影响到 joint 的两端实体
    let parent_entity = commands
        .spawn((RigidBody::Dynamic, Collider::cuboid(10f32, 10f32)))
        .id();
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(5f32, 5f32))
        .insert(ImpulseJoint::new(parent_entity, joint));

    // make a ground
    let shape_rectangle = Rectangle::new(1280., 20.);
    let mesh_handle = meshes.add(shape_rectangle);
    let color_handle = materials.add(Color::srgb(0.5, 0.4, 0.3));
    let mut transform = Transform::from_xyz(0., -START_Y + 100.0, 0.);
    transform.rotate_local_z(-0.05);
    commands
        .spawn((
            RigidBody::Fixed,
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_handle),
            transform,
            // 注意,这里没有效果.因为 ActiveEvents Component 需要放在 Collider Bundle 中
            // ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|parent| {
            let collider =
                Collider::cuboid(shape_rectangle.half_size.x, shape_rectangle.half_size.y);
            parent.spawn((
                collider,
                //ActiveEvents::COLLISION_EVENTS,
                //Name("ground".to_string()),
            ));
        });
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
