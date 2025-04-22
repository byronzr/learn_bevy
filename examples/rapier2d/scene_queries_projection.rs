///! Point Pojection 的重要特性是: 没有指定方向,只有源点,自动寻找最近的可碰撞体,进行投射
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const START_Y: f32 = 720.0 / 2.0;

#[derive(Component, Debug)]
struct TempSprite;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        RapierDebugRenderPlugin::default(),
    ));

    app.add_systems(Startup, setup);

    app.add_systems(
        Update,
        (
            make_new_collider_inside,
            make_new_collider_outside,
            projection,
        )
            .chain(),
    );

    app.add_systems(PostUpdate, show_grid);
    app.run();
}

// 创建地板
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // make ground
    make_ground(&mut commands, &mut meshes, &mut materials);
}

fn projection(
    mut commands: Commands,
    read_rapier: ReadRapierContext,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let point = Vec2::new(1., 2.);
    make_temp_sprite(&mut commands, point, Color::srgb_u8(128, 0, 0));
    let solid = true;
    let filter = QueryFilter::default();

    let rapier_context = read_rapier.single();
    if let Some((entity, projection)) = rapier_context.project_point(point, solid, filter) {
        println!("entity:{:?}, projection: {:?}", entity, projection);
        make_temp_sprite(&mut commands, projection.point, Color::srgb_u8(0, 128, 0));
    }
}

// 创建一个包含源点的碰撞体
fn make_new_collider_inside(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if !keyboard.just_pressed(KeyCode::Digit1) {
        return;
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.5, 0.4, 0.3), Vec2::splat(100.)),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        Collider::cuboid(50., 50.),
    ));
}

// 创建一个临近的碰撞体
fn make_new_collider_outside(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if !keyboard.just_pressed(KeyCode::Digit2) {
        return;
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.5, 0.4, 0.3), Vec2::splat(100.)),
        Transform::from_translation(Vec3::new(-150., -150., 0.)),
        Collider::cuboid(50., 50.),
    ));
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
    let entity = commands
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
            let entity = parent
                .spawn((
                    collider,
                    //ActiveEvents::COLLISION_EVENTS,
                    //Name("ground".to_string()),
                ))
                .id();
            println!("ground collider entity: >> {:?} <<", entity);
        })
        .id();
    println!("ground entity: >> {:?} <<", entity);
}

fn make_temp_sprite(commands: &mut Commands, pos: Vec2, color: Color) {
    // 在发射源,绘制一个图形
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(10.)),
        Transform::from_translation(pos.extend(0.0)),
        TempSprite,
    ));
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
