//! 最为常用的刚体类型
//! Dynamic,一但满足 Collider 形设的设置,将受到各种力学影响
//! Fixed, 稳如老狗
//! KinematicPositionBased, 位置动力学,不受力学影响,但可以通过代码控制位置(用得少)
//! KinematicVelocityBased, 受 Velocity 影响,不需要设定 Collider 也能够旋转(用得少)
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // NoUserData 在是力学与运动中可能会需要的自定义附加数据,但通常在学习阶段,我们并没有这么复杂的需求
    // pixels_per_meter(100.) 比较标配的设置,1米=100像素,但不适合察
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    // 这是一个调试插件,在分析碰撞与边界时,会提供一些可视化的帮助(外框)
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Startup, setup);
    app.add_systems(Update, show_grid);

    app.run();
}

// 设置
fn setup(mut world: Commands) {
    world.spawn(Camera2d);
    let type_list = [
        // 受到 Velocity 影响,如果要让它旋转,需要设定 Collider 的形状
        // 受 Dampping 影响
        // 受 Gravity 影响
        RigidBody::Dynamic,
        // 不受力学影响,但可以通过代码控制位置
        RigidBody::Fixed,
        // 不受力学影响,但可以通过代码控制位置
        RigidBody::KinematicPositionBased,
        // 只受到 Velocity 影响,
        // 并且不需要指定 Collider 也能够旋转
        RigidBody::KinematicVelocityBased,
    ];
    let dampping = Damping {
        linear_damping: 3.0,
        angular_damping: 3.0,
    };

    for (i, rigid_body_type) in type_list.iter().enumerate() {
        let transform = Transform::from_translation(Vec3::new(i as f32 * 100. - 150., 0., 0.));
        world.spawn((
            Sprite::from_color(Color::WHITE, Vec2::splat(30.)),
            transform,
            rigid_body_type.clone(),
            // 阻尼
            dampping,
            // 速率
            Velocity {
                linvel: Vec2::new(1., 1.),
                angvel: 1.,
            },
            // 碰撞体形状
            Collider::cuboid(15., 15.),
            // 重力
            GravityScale(1.0),
            // (碰撞体)质量
            ColliderMassProperties::Density(1.),
        ));
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
