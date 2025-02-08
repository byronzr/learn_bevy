//! This example demonstrates bounding volume intersections.

use bevy::{
    color::palettes::css::*,
    math::{bounding::*, ops, Isometry2d},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Test>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_text, spin, update_volumes, update_test_state),
        )
        .add_systems(
            PostUpdate, // 在 Update 后执行
            (
                render_shapes,
                // 碰撞测试,择一运行,并没有相应的射线实体(数据),
                // 在这里,驱动射线的数据是时间转换的三角函数值
                (
                    aabb_intersection_system.run_if(in_state(Test::AabbSweep)), // 矩形扫描测试
                    circle_intersection_system.run_if(in_state(Test::CircleSweep)), // 圆形扫描测试
                    ray_cast_system.run_if(in_state(Test::RayCast)),            // 纯射线测试
                    aabb_cast_system.run_if(in_state(Test::AabbCast)),          // (矩形)射线测试
                    bounding_circle_cast_system.run_if(in_state(Test::CircleCast)), // (圆形)射线测试
                ),
                render_volumes,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Spin;

/// 驱动所有 Spin 标记的 Shape 自旋转
fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 5.);
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Test {
    AabbSweep,   // 方形边界滑动测试
    CircleSweep, // 圆形边界滑动测试
    #[default]
    RayCast, // 射线测试,不限制外框
    AabbCast,    // 方形边界有效,射线测试
    CircleCast,  // 圆形边界有效,射线测试
}

/// 更新射线上的碰撞体
/// (无渲染)
fn update_test_state(
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<Test>>,
    mut state: ResMut<NextState<Test>>,
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }

    use Test::*;
    let next = match **cur_state {
        AabbSweep => CircleSweep,
        CircleSweep => RayCast,
        RayCast => AabbCast,
        AabbCast => CircleCast,
        CircleCast => AabbSweep,
    };
    state.set(next);
}

/// 更新控制菜单的文本
fn update_text(mut text: Single<&mut Text>, cur_state: Res<State<Test>>) {
    if !cur_state.is_changed() {
        return;
    }

    text.clear();

    text.push_str("Intersection test:\n");
    use Test::*;
    for &test in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
        let s = if **cur_state == test { "*" } else { " " };
        text.push_str(&format!(" {s} {test:?} {s}\n"));
    }
    text.push_str("\nPress space to cycle");
}

#[derive(Component)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}

/// 使用 gizmos 绘制所有 Shape
/// (渲染)
fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(r, isometry, color);
            }
            Shape::Circle(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(t, isometry, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(l, isometry, color);
            }
            Shape::Capsule(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }
            Shape::Polygon(p) => {
                gizmos.primitive_2d(p, isometry, color);
            }
        }
    }
}

#[derive(Component)]
enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Debug)]
enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

/// 为所有 Entity 更新体积(可碰撞测试区域),因为 Entity 一直在旋转,
/// 用不断的 insert 进行 CurrentVolume 的更新
/// (无渲染)
fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &DesiredVolume, &Shape, &Transform),
        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, desired_volume, shape, transform) in query.iter() {
        // 实体坐标点
        let translation = transform.translation.xy();
        // 实体 2D 平面旋转角度(Z轴)
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        // 当前实体,用于碰撞测试的2D刚体等距信息(位置与角度)
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        // ** 实体的可碰撞测试区域组件
        // ** 内置的体积组件,只有 aabb_2d 与 bounding_circle 两种,
        // ** 如果更细致的碰撞测试,需要自定义体积组件,获用第三方库了
        match desired_volume {
            // Aabb (Axis-Aligned Bounding Box) 为了简化计算,使用与坐标轴平行的矩形
            // 矩形
            DesiredVolume::Aabb => {
                let aabb = match shape {
                    Shape::Rectangle(r) => r.aabb_2d(isometry),
                    Shape::Circle(c) => c.aabb_2d(isometry),
                    Shape::Triangle(t) => t.aabb_2d(isometry),
                    Shape::Line(l) => l.aabb_2d(isometry),
                    Shape::Capsule(c) => c.aabb_2d(isometry),
                    Shape::Polygon(p) => p.aabb_2d(isometry),
                };
                // ** replace
                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
            }
            // 圆形
            DesiredVolume::Circle => {
                let circle = match shape {
                    Shape::Rectangle(r) => r.bounding_circle(isometry),
                    Shape::Circle(c) => c.bounding_circle(isometry),
                    Shape::Triangle(t) => t.bounding_circle(isometry),
                    Shape::Line(l) => l.bounding_circle(isometry),
                    Shape::Capsule(c) => c.bounding_circle(isometry),
                    Shape::Polygon(p) => p.bounding_circle(isometry),
                };
                commands
                    .entity(entity)
                    .insert(CurrentVolume::Circle(circle));
            }
        }
    }
}

/// 使用 Gizmos 绘制所有 Entity 的可碰撞区域
/// (渲染)
fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        // 如果碰撞改变颜色
        let color = if **intersects { AQUA } else { ORANGE_RED };
        match volume {
            CurrentVolume::Aabb(a) => {
                gizmos.rect_2d(a.center(), a.half_size() * 2., color);
            }
            CurrentVolume::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
            }
        }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
struct Intersects(bool);

const OFFSET_X: f32 = 125.;
const OFFSET_Y: f32 = 75.;

/// 初始化六个 Shape 与 文本说明
/// (无渲染)
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
        Shape::Circle(Circle::new(45.)), // 形状
        DesiredVolume::Aabb,             // 期望的体积
        Intersects::default(),           // 是否相交
    ));

    commands.spawn((
        Transform::from_xyz(0., OFFSET_Y, 0.),
        Shape::Rectangle(Rectangle::new(80., 80.)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
        Shape::Triangle(Triangle2d::new(
            Vec2::new(-40., -40.),
            Vec2::new(-20., 40.),
            Vec2::new(40., 50.),
        )),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
        Shape::Line(Segment2d::new(Dir2::from_xy(1., 0.3).unwrap(), 90.)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(0., -OFFSET_Y, 0.),
        Shape::Capsule(Capsule2d::new(25., 50.)),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
        Shape::Polygon(RegularPolygon::new(50., 6)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

/// 绘制小圆点,用于射线碰撞测试
fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
    for r in [1., 2., 3.] {
        gizmos.circle_2d(position, r, color);
    }
}

/// 绘制射线
fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        WHITE,
    );
    // 发射端小圆点,紫红色
    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
}

/// 创建并绘制射线
fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
    // ** 如果基于中心点,ray是一个向量
    // ** 其本身只是一个坐标点
    // ** sin 与 cos 的求值范围只有 [-1,1]之间,
    // ** 所以几乎肉眼认为就是中心点通常需要放大
    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
    // 射线长度
    let dist = 150. + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.;

    // 创建一个射线(数据)
    let aabb_ray = Ray2d {
        origin: ray * 250.,                   // ray 本身值太小了,放大 250倍,使期偏移
        direction: Dir2::new_unchecked(-ray), // 再将其投射到反方向
    };
    // 基于数据构建可碰撞的射线
    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);
    // 绘制射线
    draw_ray(gizmos, &ray_cast);
    ray_cast
}

/// 纯射线横断测试
fn ray_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    // 绘制射线并获得射线引用
    // 并不需要将它 spawn 进 world
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);

    for (volume, mut intersects) in volumes.iter_mut() {
        // 对实体进行碰撞测试
        let toi = match volume {
            CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
            CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
        };
        // 如果 toi 有值说明相交点在射线上
        **intersects = toi.is_some();
        // 如果有相交点,绘制一个小圆点(绿黄色)
        if let Some(toi) = toi {
            draw_filled_circle(
                &mut gizmos,
                ray_cast.ray.origin + *ray_cast.ray.direction * toi,
                LIME,
            );
        }
    }
}

/// 与矩形刚体相交,相交点绘制贴边矩形
fn aabb_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);

    // ** 与纯射不同的地方是,
    // ** 这里的射线,可以想像一下,
    // ** 发射出去的不是粒子,而是一个个矩形(Aabb)
    // ** 所以这个射线在作碰撞测试时,有了 "宽度".
    let aabb_cast = AabbCast2d {
        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)), // 15 可以简单理解为宽度
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            // ** aabb_collison_at 也说明它与 aabb_intersection_at 的区别
            CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
            CurrentVolume::Circle(_) => None,
        };

        **intersects = toi.is_some();

        // ** 绘制碰撞刚体(aabb)
        if let Some(toi) = toi {
            gizmos.rect_2d(
                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
                aabb_cast.aabb.half_size() * 2.,
                LIME,
            );
        }
    }
}

/// 也 aabb_cast_system 类似,只不过碰撞体为圆形
fn bounding_circle_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    let circle_cast = BoundingCircleCast {
        circle: BoundingCircle::new(Vec2::ZERO, 15.),
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            CurrentVolume::Aabb(_) => None,
            CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
        };

        **intersects = toi.is_some();
        if let Some(toi) = toi {
            gizmos.circle_2d(
                circle_cast.ray.ray.origin + *circle_cast.ray.ray.direction * toi,
                circle_cast.circle.radius(),
                LIME,
            );
        }
    }
}

/// 根据时间获得一个扫描中心点,主要用于两个扫描窗
fn get_intersection_position(time: &Time) -> Vec2 {
    let x = ops::cos(0.8 * time.elapsed_secs()) * 250.;
    let y = ops::sin(0.4 * time.elapsed_secs()) * 100.;
    Vec2::new(x, y)
}

/// aabb 横断面测试
fn aabb_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    // 根据时间获得一个扫描中心点
    let center = get_intersection_position(&time);
    // 创建一个用于扫描的刚体 Aabb,中心点为 center,半径为 50
    let aabb = Aabb2d::new(center, Vec2::splat(50.));
    // gizmos 绘制(将刚体可视化)
    gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);
    // 进行碰撞测试
    for (volume, mut intersects) in volumes.iter_mut() {
        // 判断是否相交
        // 将实体的碰撞体积置入扫描体积中进行碰撞测试
        let hit = match volume {
            CurrentVolume::Aabb(a) => aabb.intersects(a),
            CurrentVolume::Circle(c) => aabb.intersects(c),
        };
        // 更新实体的碰撞测试结果
        **intersects = hit;
    }
}

/// 与 aabb 相似,只不过扫描窗为圆形
fn circle_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let center = get_intersection_position(&time);
    let circle = BoundingCircle::new(center, 50.);
    gizmos.circle_2d(center, circle.radius(), YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            CurrentVolume::Aabb(a) => circle.intersects(a),
            CurrentVolume::Circle(c) => circle.intersects(c),
        };

        **intersects = hit;
    }
}
