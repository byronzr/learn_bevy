use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const RADIUS: f32 = 300.;

#[derive(Component)]
struct MajorStar;

#[derive(Component)]
struct MinorStar;

#[derive(Resource, Default)]
struct Running(bool);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.init_resource::<Running>();

    // setup
    app.add_systems(Startup, (setup, show_grid));

    app.add_systems(FixedUpdate, (circum, space));

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let r_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-50., 0.))
        .local_anchor2(Vec2::new(150., 0.))
        // target_vel = 弧度值/秒 (正值为逆时针, 负值为顺时针)
        // 当扭力大于阻力时,关节完成一次转动后,后续可能进入一种平衡状态.
        // 所以, 可以认为 factor 是一个初始速度
        .motor_velocity(-30., 10.0);

    let p_joint = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(-50., 0.))
        .local_anchor2(Vec2::new(20., 0.))
        .limits([-100., 100.])
        // target_pos 在 RevoluteJoint 与 PrismaticJoint 中是不同的
        // 弧度值 / 目标位置
        // stiffness 刚性值,提供类似扭力的作用(在PrismaticJoint中是弹簧的刚性)
        .motor_position(30., 1000.0, 0.0);

    // Major yellow
    let major = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            //RigidBody::Fixed,
            Collider::ball(15.),
            Mesh2d(meshes.add(Circle::new(15.))),
            MeshMaterial2d(materials.add(Color::srgb_u8(128, 128, 0))),
            MajorStar,
            Transform::from_xyz(RADIUS, 0., 0.),
        ))
        .id();

    // Minor blue
    commands.spawn((
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Circle::new(10.))),
        MeshMaterial2d(materials.add(Color::srgb_u8(0, 0, 128))),
        MinorStar,
        ImpulseJoint::new(major, r_joint),
        // 设置了 Collider, joint 才可以驱动它,
        // 如果不是一个 colider 那么 rapier 不知道力学公式如何影响它
        Collider::ball(10.),
        GravityScale(0.0),
    ));

    // Minor teal
    let minor_teal = commands
        .spawn((
            RigidBody::Dynamic,
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(Color::srgb_u8(0, 128, 128))),
            MinorStar,
            ImpulseJoint::new(major, p_joint),
            // 设置了 Collider, joint 才可以驱动它,
            // 如果不是一个 colider 那么 rapier 不知道力学公式如何影响它
            Collider::ball(10.),
        ))
        .id();

    // 过于复杂的 joint 可能会导致物理引擎计算混乱
    // Minor teal
    commands.spawn((
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(materials.add(Color::srgb_u8(128, 0, 128))),
        MinorStar,
        ImpulseJoint::new(minor_teal, p_joint),
        // 设置了 Collider, joint 才可以驱动它,
        // 如果不是一个 colider 那么 rapier 不知道力学公式如何影响它
        Collider::ball(5.),
    ));
}

// 环绕
fn circum(
    query: Single<&mut Transform, With<MajorStar>>,
    time: Res<Time>,
    running: ResMut<Running>,
) {
    if !running.0 {
        return;
    }
    let mut transform = query.into_inner();
    let angle = time.elapsed_secs() / 100. * std::f32::consts::TAU;
    let (x, y) = (RADIUS * angle.cos(), RADIUS * angle.sin());

    transform.translation = Vec3::new(x, y, 0.0);
}

fn space(mut running: ResMut<Running>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        running.0 = !running.0;
    }
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
    gizmos.circle_2d(
        Vec2::new(0., 0.),               // 圆心
        RADIUS,                          // 半径
        Color::srgba_u8(128, 0, 0, 128), // 圆圈颜色
    );
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
