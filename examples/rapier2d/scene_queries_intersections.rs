///! intersections_with_point,用点去测试物体
///! intersections_with_shape,用物体去测试点
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const START_Y: f32 = 720.0 / 2.0;

#[derive(Component, Debug)]
struct BindObjective;

#[derive(Resource, Debug, Default)]
enum BindType {
    #[default]
    Shape,
    Point,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        RapierDebugRenderPlugin::default(),
    ));

    app.init_resource::<BindType>();

    app.add_systems(Startup, (usage, setup, show_grid).chain());

    app.add_systems(Update, (switch, movement, intersection_test).chain());

    app.add_systems(PostUpdate, show_grid);
    app.run();
}

fn intersection_test(
    reader_rapier: ReadRapierContext,
    bind_type: Res<BindType>,
    mut reader_cur: EventReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) -> Result {
    let Some(event) = reader_cur.read().last() else {
        return Ok(());
    };
    let cursor_pos = event.position;
    let (camera, global_transform) = *camera;
    let Ok(world_pos) = camera.viewport_to_world_2d(global_transform, cursor_pos) else {
        return Ok(());
    };
    let rapier_context = reader_rapier.single()?;
    let filter = QueryFilter::default();
    match *bind_type {
        BindType::Point => {
            rapier_context.intersections_with_point(world_pos, QueryFilter::default(), |entity| {
                println!(
                    "[{:?}] point >> intersection entity: {:?}",
                    time.elapsed_secs(),
                    entity
                );
                true
            });
        }

        BindType::Shape => {
            let shape = Collider::cuboid(10., 10.);
            let shape_rot = 0.0;
            rapier_context.intersections_with_shape(
                Vec2::default(),
                shape_rot,
                &shape,
                filter,
                |entity| {
                    println!(
                        "[{:?}] shape >> intersection entity: {:?}",
                        time.elapsed_secs(),
                        entity
                    );
                    true
                },
            );
        }
    }
    Ok(())
}

// default
fn switch(mut bt: ResMut<BindType>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        match *bt {
            BindType::Point => {
                *bt = BindType::Shape;
                println!("bind type: Shape");
            }
            BindType::Shape => {
                *bt = BindType::Point;
                println!("bind type: Point");
            }
        }
    }
}

fn movement(
    mut commands: Commands,
    bt: Res<BindType>,
    mut reader_cur: EventReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    bo: Single<Entity, With<BindObjective>>,
) {
    match *bt {
        BindType::Point => {
            // println!("bind type: Point");
            // to nothing
        }
        BindType::Shape => {
            if let Some(event) = reader_cur.read().last() {
                let cursor_pos = event.position;
                let (camera, global_transform) = *camera;
                if let Ok(world_pos) = camera.viewport_to_world_2d(global_transform, cursor_pos) {
                    let transform = Transform::from_xyz(world_pos.x, world_pos.y, 0.);
                    commands.entity(*bo).insert(transform);
                }
            }
        }
    };
}

// 创建地板
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut text: Single<&mut Text>,
) {
    commands.spawn(Camera2d);

    // make bind objective
    let entity = commands
        .spawn((Collider::cuboid(10., 10.), BindObjective))
        .id();
    println!("Bind Objective: {:?}", entity);

    text.0
        .push_str("Press Space to switch between Point and Shape");

    // make ground
    make_ground(&mut commands, &mut meshes, &mut materials);
}

// 创建地板
fn make_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // make a ground
    let shape_rectangle = Rectangle::new(1280., 20.);
    let mesh_handle = meshes.add(shape_rectangle);
    let color_handle = materials.add(Color::srgb(0.5, 0.4, 0.3));
    let mut transform = Transform::from_xyz(0., -START_Y + 100.0, 0.);
    transform.rotate_local_z(-0.05);
    commands.spawn((
        RigidBody::Fixed,
        Mesh2d(mesh_handle),
        MeshMaterial2d(color_handle),
        transform,
        children![(Collider::cuboid(
            shape_rectangle.half_size.x,
            shape_rectangle.half_size.y
        ),)],
    ));
}

fn usage(mut commands: Commands) {
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

    let mut gizmos = GizmoAsset::default();
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::splat(20.),
        Color::srgba_u8(255, 0, 0, 155),
    );
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
