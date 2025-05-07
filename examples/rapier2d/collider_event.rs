/// ! ActiveEvents::COLLISION_EVENTS,碰撞事件(提高性能消耗)
/// ! ActiveEvents::CONTACT_FORCE_EVENTS,接触力事件(性能消耗更大)
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

const START_X: f32 = 1280.0 / 2.0;
const START_Y: f32 = 720.0 / 2.0;

#[derive(Component, Debug)]
struct Name(String);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_systems(Startup, (setup, show_grid));

    // 需要对应用的 Collider Bundle 中有 ActiveEvents::COLLISION_EVENTS, 才能触发事件
    app.add_systems(Update, events_reader);
    // 需要对应用的 Collider Bundle 中有 ActiveEvents::CONTACT_FORCE_EVENTS, 才能触发事件
    app.add_systems(Update, contact_reader);

    app.run();
}

// 碰撞事件
fn events_reader(mut events_reader: EventReader<CollisionEvent>, query: Query<(Entity, &Name)>) {
    for event in events_reader.read() {
        match event {
            CollisionEvent::Started(a, b, flags) => {
                let Ok(a_value) = query.get(*a) else {
                    continue;
                };
                let Ok(b_value) = query.get(*b) else {
                    continue;
                };

                let ty = match *flags {
                    CollisionEventFlags::SENSOR => "sensor",
                    CollisionEventFlags::REMOVED => "removed",
                    _ => "unknown",
                };

                println!(
                    "collision started: {} -> {} ({:?})",
                    a_value.1.0, b_value.1.0, ty,
                );
            }
            CollisionEvent::Stopped(a, b, flags) => {
                let Ok(a_value) = query.get(*a) else {
                    continue;
                };
                let Ok(b_value) = query.get(*b) else {
                    continue;
                };
                let ty = match *flags {
                    CollisionEventFlags::SENSOR => "sensor",
                    CollisionEventFlags::REMOVED => "removed",
                    _ => "unknown",
                };
                println!(
                    "collision stopped: {} -> {} ({:?})",
                    a_value.1.0, b_value.1.0, ty,
                );
            }
        }
    }
}

// 接触力事件
fn contact_reader(
    mut contact_reader: EventReader<ContactForceEvent>,
    query: Query<(Entity, &Name)>,
) {
    for event in contact_reader.read() {
        let ContactForceEvent {
            collider1,
            collider2,
            total_force,
            total_force_magnitude,
            max_force_direction,
            max_force_magnitude,
        } = *event;

        let Ok(c1_value) = query.get(collider1) else {
            continue;
        };
        let Ok(c2_value) = query.get(collider2) else {
            continue;
        };

        println!(
            "contact force: c1({}) -> c2({})",
            c1_value.1.0, c2_value.1.0
        );
        println!("  total force: {:?}", total_force);
        println!("  total force magnitude: {:?}", total_force_magnitude);
        println!("  max force direction: {:?}", max_force_direction);
        println!("  max force magnitude: {:?}", max_force_magnitude);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // make a lot of dynamic bodies
    let shape_polygon = RegularPolygon::new(10., 9);
    let vetexes = shape_polygon.vertices(0.).into_iter().collect::<Vec<_>>();
    let mesh_handle = meshes.add(shape_polygon);
    let color_handle = materials.add(Color::srgb(0.8, 0.7, 0.6));
    // 为了方便观察,可以将计数调成 1,只观察一个碰撞体的事件
    for i in 0..10 {
        let x = -START_X + (i + 1) as f32 * 30.0;
        let y = START_Y - 100.0;
        let transform = Transform::from_xyz(x, y, 0.);
        commands
            .spawn((
                RigidBody::Dynamic,
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(color_handle.clone()),
                transform,
            ))
            .with_children(|parent| {
                let Some(collider) = Collider::convex_hull(&vetexes) else {
                    return;
                };
                parent.spawn((
                    collider,
                    Name(format!("shape-{}", i)),
                    // 在此处再次添加 ActiveEvents 能够使球体之间的碰撞发生事件,
                    // ActiveEvents::COLLISION_EVENTS,
                    // ActiveEvents::CONTACT_FORCE_EVENTS,

                    // 当有足够的弹性时,ActiveEvents::COLLISION_EVENTS 会触发(Stoped)
                    Restitution::new(0.7),
                    Friction::new(0.01),
                    ColliderMassProperties::Mass(100.),
                    // 传感器触发的事件与碰撞体触发的事件相似
                    // Sensor,
                ));
            });
    }

    // make a ground
    // 地板
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
                ActiveEvents::CONTACT_FORCE_EVENTS,
                Name("ground".to_string()),
            ));
        });
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
