//! 最为常用的刚体类型
//! 以 Dynamic 为例,
#![allow(dead_code)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 创建一个资源,用于存储实体
// 因为设置需要有序进行,如果使用 Query 会导致无法预测
#[derive(Resource, Debug, Default)]
struct Entities(Vec<Entity>);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // NoUserData 在是力学与运动中可能会需要的自定义附加数据,但通常在学习阶段,我们并没有这么复杂的需求
    // pixels_per_meter(100.) 比较标配的设置,1米=100像素,但不适合察
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    // 这是一个调试插件,在分析碰撞与边界时,会提供一些可视化的帮助(外框)
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.init_resource::<Entities>();

    // 统一设置了无重力影响,无质量的刚体
    app.add_systems(Startup, setup);
    app.add_systems(Update, show_grid);

    // 添加质量
    // 无质量(0)刚体,不受力学影响
    app.add_systems(Update, mass);

    // 添加重力影响
    // 无重力(0)刚体,受力学影响,但无初速度,不会动
    app.add_systems(Update, gravity);

    // 添加持续的外力(向左,扭矩为逆时针旋转)
    // 持续的力,会是一个保持的值,60桢内一直是这个值
    app.add_systems(Update, external_force);

    // 添加脉冲力
    // 脉冲力,是一个会被刚体吸收的值,60桢内这个值都会被吸收,
    // 所以,虽然脉冲的 value 的基数是 500,但每桢都喂了 500 的基数,
    // 很快会超过它
    app.add_systems(Update, external_impluse);

    // 添加阻尼
    // 阻尼同样会影响重力
    // app.add_systems(Update, damping);

    // 优势组
    // 正值大于负值的方向,所以开启优势组以后,
    // 观察配合连续脉冲波(has_run注释掉),是撞不开第一个方块的
    app.add_systems(Update, dominance);

    // 最后,请观察,第一个方块.
    // 在 Debug 方式下,能看到外边框在一定时间后,由红转黑,
    // 这说明,该物体因为闲置,被系统标记为 Sleeping 以节约算力

    app.run();
}

/// 优势组
fn dominance(entities: Res<Entities>, mut commands: Commands, mut has_run: Local<bool>) {
    if *has_run {
        return;
    }
    *has_run = true;

    for (i, entity) in entities.0.iter().enumerate() {
        let value = 0 - i as i8;
        commands.entity(*entity).insert(Dominance::group(value));
    }
}

/// 阻尼
fn damping(entities: Res<Entities>, mut commands: Commands, mut has_run: Local<bool>) {
    if *has_run {
        return;
    }
    // 只运行一次,这样,阻尼就会慢慢的消失
    // 不然,每桢都运行,就会一直有阻尼(新的)
    *has_run = true;

    for (i, entity) in entities.0.iter().enumerate() {
        let _value = i as f32;
        commands.entity(*entity).insert(Damping {
            linear_damping: 5.,
            angular_damping: 5.,
        });
    }
}

/// 脉冲波
fn external_impluse(entities: Res<Entities>, mut commands: Commands, mut has_run: Local<bool>) {
    // 只运行一次
    if *has_run {
        return;
    }
    // 启用,则每桢就会发送一个脉冲波
    *has_run = true;
    for (i, entity) in entities.0.iter().enumerate() {
        let value = i as f32 * 500.0;
        commands.entity(*entity).insert(ExternalImpulse {
            impulse: Vec2::new(-value, 0.),
            torque_impulse: value,
        });
    }
}

/// 外力
fn external_force(entities: Res<Entities>, mut commands: Commands, mut has_run: Local<bool>) {
    // 只运行一次
    if *has_run {
        return;
    }
    *has_run = true;
    for (i, entity) in entities.0.iter().enumerate() {
        let value = i as f32 * 5000.0;
        commands.entity(*entity).insert(ExternalForce {
            force: Vec2::new(-value, 0.),
            torque: value,
        });
    }
}

/// 质量
fn mass(entities: Res<Entities>, mut commands: Commands, mut has_run: Local<bool>) {
    // 只运行一次
    if *has_run {
        return;
    }
    *has_run = true;
    for (_i, entity) in entities.0.iter().enumerate() {
        let value = 1.0;

        // 加大这个值,自已试试
        // let value = i as f32 * 1.0;
        commands
            .entity(*entity)
            .insert(ColliderMassProperties::Density(value));
    }
}

/// 重力
fn gravity(entities: Res<Entities>, mut commands: Commands) {
    for (i, entity) in entities.0.iter().enumerate() {
        commands.entity(*entity).insert(GravityScale(1. * i as f32));
    }
}

// 设置
// 创建 10 个 rigid body, 以便观察,虽然展示的是 Rigid 相关的效果
fn setup(mut world: Commands, mut entities: ResMut<Entities>) {
    world.spawn(Camera2d);

    for i in 0..10 {
        let transform = Transform::from_translation(Vec3::new(i as f32 * 100. - 500., 300., 0.));
        let entity = world
            .spawn((
                Sprite::from_color(Color::WHITE, Vec2::splat(30.)),
                transform,
                RigidBody::Dynamic,
                // 碰撞体形状
                Collider::cuboid(15., 15.),
                // 无重力
                GravityScale(0.0),
                // (碰撞体)无质量
                ColliderMassProperties::Density(0.),
            ))
            .id();
        entities.0.push(entity);
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
