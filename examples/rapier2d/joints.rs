#![allow(dead_code)]
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const START_X: f32 = 1280.0 / 2.0;
const START_Y: f32 = 720.0 / 2.0;

// 受力体标识
#[derive(Component, Debug)]
struct Objective;

#[derive(Resource, Debug)]
struct ImpluseTimer(Timer);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(ImpluseTimer(Timer::new(
        Duration::from_secs(1),
        TimerMode::Repeating,
    )));

    // setup
    app.add_systems(Startup, setup);

    // external force
    app.add_systems(FixedUpdate, external_impluse);

    app.add_systems(PostUpdate, show_grid);
    app.run();
}

fn external_impluse(
    mut commands: Commands,
    query: Query<Entity, With<Objective>>,
    mut timer: ResMut<ImpluseTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for entity in query.iter() {
            commands.entity(entity).insert(ExternalImpulse {
                impulse: Vec2::new(-50_000., 50_000.),
                torque_impulse: 0.0,
            });
        }
        println!("external_impluse");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let start_x = -500.; // 起点
    let group_width = 200.; // 组宽
    let gap = 300.; // 间隔

    // ! joint 首先独立的,以 (0,0) 为中心点来确定了两个,anchor的位置相对关系
    // ! joint 不会控制整体的 position,整体的 position 只会受到 anchor1 的影响
    // ! 所以,在这里准备好所有的 position
    // local_anchor1 通常是当前实体
    // local_anchor2 通常是父级实体
    // joint 只是建立了关系,并不会有实体的"线"存在,所以它可以穿过任何实体
    // DOF: Degree of Freedom 自由度的概念,在 Joint 中就是限制 DOF
    let mut vec_positions = vec![];
    for i in 0..5 {
        let anchor1 = start_x + i as f32 * group_width + gap as f32;
        vec_positions.push(Vec2::new(anchor1, 0.0));
    }

    vec_positions.reverse();

    println!("vec_positions: {:?}", vec_positions);

    // * -- FixedJoint -- *
    let joint = FixedJointBuilder::new()
        .local_anchor1(Vec2 { x: 30., y: 0. })
        .local_anchor2(Vec2 { x: -30., y: 0. });

    // local_anchor2
    let parent_entity = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(10f32, 10f32),
            Objective,
        ))
        .id();

    // local_anchor1
    commands.spawn((
        RigidBody::Fixed,
        //RigidBody::Dynamic,
        Collider::cuboid(5f32, 5f32),
        ImpulseJoint::new(parent_entity, joint),
        Transform::from_translation(vec_positions.pop().unwrap().extend(0.)),
    ));

    // * -- PrismaJoint -- *
    // 消除 X 轴限制
    let joint = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2 { x: 30., y: 0. })
        .local_anchor2(Vec2 { x: -30., y: 0. });

    // local_anchor2
    let parent_entity = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(10f32, 10f32),
            Objective,
        ))
        .id();

    // local_anchor1
    commands.spawn((
        RigidBody::Fixed,
        //RigidBody::Dynamic,
        Collider::cuboid(5f32, 5f32),
        ImpulseJoint::new(parent_entity, joint),
        Transform::from_translation(vec_positions.pop().unwrap().extend(0.)),
    ));

    // * -- RevoluteJoint -- *
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2 { x: 30., y: 0. })
        .local_anchor2(Vec2 { x: -30., y: 0. });
    //.motor_position(target_pos, 0., 0.);

    // local_anchor2
    let parent_entity = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(10f32, 10f32),
            Objective,
        ))
        .id();

    // local_anchor1
    commands.spawn((
        RigidBody::Fixed,
        //RigidBody::Dynamic,
        Collider::cuboid(5f32, 5f32),
        ImpulseJoint::new(parent_entity, joint),
        Transform::from_translation(vec_positions.pop().unwrap().extend(0.)),
    ));

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
