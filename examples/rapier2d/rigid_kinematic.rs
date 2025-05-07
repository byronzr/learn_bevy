use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const RADIUS: f32 = 15.0;

#[derive(Component, Debug)]
struct Target;

#[derive(Component, Debug)]
struct MarkerPosition;

#[derive(Component, Debug)]
struct MarkerVelocity;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        RapierDebugRenderPlugin::default(),
    ));

    app.add_systems(Startup, (setup, show_grid));

    app.add_systems(Update, (space_controller, draw_position, set_velocity));

    app.run();
}

// 设置
fn setup(mut world: Commands) {
    world.spawn(Camera2d);

    // kinematic position
    world.spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(RADIUS * 2.)),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(RADIUS, RADIUS),
        GravityScale(1.),
        ColliderMassProperties::Density(1.),
        Velocity {
            linvel: Vec2::new(100., 0.),
            angvel: 0.,
        },
        MarkerPosition,
        Transform::from_xyz(0., 0., 0.),
    ));

    // kinematic velocity
    world.spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(RADIUS * 2.)),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(RADIUS, RADIUS),
        GravityScale(1.),
        ColliderMassProperties::Density(1.),
        Velocity::default(),
        MarkerVelocity,
        Transform::from_xyz(-50., -200., 0.),
    ));

    // left cuboid
    world.spawn((
        Sprite::from_color(Color::srgb_u8(128, 64, 32), Vec2::splat(RADIUS * 2.)),
        RigidBody::Dynamic,
        Collider::cuboid(RADIUS, RADIUS),
        GravityScale(0.),
        ColliderMassProperties::Density(1.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        Target,
        Transform::from_xyz(-50., 0., 0.),
    ));

    // right cuboid
    world.spawn((
        Sprite::from_color(Color::srgb_u8(128, 64, 32), Vec2::splat(RADIUS * 2.)),
        RigidBody::Dynamic,
        Collider::cuboid(RADIUS, RADIUS),
        GravityScale(0.),
        ColliderMassProperties::Density(1.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        Target,
        Transform::from_xyz(100., 0., 0.),
    ));
}

// serve position
// 1. 第一次按下 D 键,可使控制物与右边的碰撞体发生重叠(但不会击活碰撞事件,如果数值未重叠则挤开物体)
// 2. 当第二次按下 D/F 键,时才计算各种力学碰撞
// 3. 以此测试,得出结果,position 的设置在 start 阶段,会触发碰撞,而 end 阶段,只会(堆放).
// 4. start 阶段没有接触(contact)的物体,则不会发生任何事情
fn draw_position(
    mut query: Query<(&RigidBody, &mut Transform), With<MarkerPosition>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyD) && !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let value = if keyboard.just_pressed(KeyCode::KeyD) {
        // 会与物体重叠
        // 100.

        // 会将物体挤开
        90.
    } else {
        5.
    };

    for (_body, mut transform) in &mut query {
        transform.translation.x += value;
    }
}

// serve position
// 不断的按下空格键,让物体向左移动,可以看到控制物体穿过了左边的碰撞体
fn space_controller(
    mut query: Query<(&RigidBody, &mut Transform), With<MarkerPosition>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for (_body, mut transform) in &mut query {
        transform.translation.x -= 100.;
    }
}

// serve velocity
// Velocity 测试,让下面的物体得到一个速率,并生成一个相同的参照物(Dynamic),观察两者的差异
fn set_velocity(
    mut commands: Commands,
    mut query: Query<(Entity, &RigidBody, &mut Velocity), With<MarkerVelocity>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyV) {
        return;
    }
    for (entity, _body, mut velocity) in &mut query {
        velocity.linvel = Vec2::new(0., 20.);
        velocity.angvel = 10.;
        commands.entity(entity).insert(Damping {
            linear_damping: 5.,
            angular_damping: 5.,
        });
    }

    // 同时放入一个样本
    commands.spawn((
        Sprite::from_color(Color::srgb_u8(32, 128, 32), Vec2::splat(RADIUS * 2.)),
        RigidBody::Dynamic,
        Collider::cuboid(RADIUS, RADIUS),
        // 发挥作用,为了让它也向上保持运动,重力约束设为了 0.
        GravityScale(0.),
        ColliderMassProperties::Density(1.),
        Velocity {
            linvel: Vec2::new(0., 20.),
            angvel: 10.,
        },
        // 发挥作用,很快就会停止,注释掉能与控制体保持一致的移动与转速
        Damping {
            linear_damping: 5.,
            angular_damping: 5.,
        },
        Transform::from_xyz(-150., -200., 0.),
    ));
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
