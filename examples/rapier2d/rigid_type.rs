//! 最为常用的刚体类型
//! Dynamic,受到各种力学影响
//! Fixed, 稳如老狗
//! KinematicPositionBased, 用户指定 Position,其它力学为系统推断
//! KinematicVelocityBased, 用户指定 Velocity,其它力学为系统推断
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
        // 不受力学影响
        RigidBody::Fixed,
        // 由用户指定位置(position),从而推断出相应力学参数(未证实)
        // 因为没有指定位置,所以展示中状态如 Fixed
        RigidBody::KinematicPositionBased,
        // 由用户指定加速(velocity),从而推断出相应力学参数(未证实)
        // 因为指定了加速,向量为(1,1),所以会旋转并缓慢向右上角移动
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
            // Dynamic 受到影响
            // KinematicVelocityBased 不受影响,
            Collider::cuboid(15., 15.),
            // 重力
            GravityScale(1.0),
            // Mass(质量),Density(密度)
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
