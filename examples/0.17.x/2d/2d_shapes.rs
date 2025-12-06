//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
    ))
    .insert_resource(ClearColor(Color::BLACK))
    //.insert_resource(ClearColor(Color::WHITE))
    .add_systems(Startup, setup);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 创建图形网络 (mesh) 并添加到场景中
    let shapes = [
        meshes.add(Circle::new(50.0)),                // 圆形
        meshes.add(CircularSector::new(50.0, 1.0)),   // 扇形 (max angle(弧度值) = 2pi)
        meshes.add(CircularSegment::new(50.0, 1.25)), // 圆弧
        meshes.add(Ellipse::new(25.0, 50.0)),         // 椭圆
        meshes.add(Annulus::new(25.0, 50.0)), // 圆环 (inner radius = 25.0, outer radius = 50.0)
        meshes.add(Capsule2d::new(25.0, 50.0)), // 胶囊
        meshes.add(Rhombus::new(75.0, 100.0)), // 棱形
        meshes.add(Rectangle::new(50.0, 100.0)), // 矩形
        meshes.add(RegularPolygon::new(50.0, 6)), // 多边形
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )), // 三角形
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        // hue 为色相, saturation 为饱和度, lightness 为亮度
        // hue 是色轮,最大取值为 360 度, 0 和 360 是红色, 120 是绿色, 240 是蓝色
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape), // 将网格组件化
            //MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press <space> to toggle wireframes\nPress <c> to toggle material colors"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<EntityRef, With<Mesh2d>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 展示线框图(meshes.add)
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
    // 切换材质(颜色)
    if keyboard.just_pressed(KeyCode::KeyC) {
        let num_shapes = query.iter().len();
        for (i, entity_ref) in query.iter().enumerate() {
            let mut entity = commands.entity(entity_ref.id());
            if entity_ref.get::<MeshMaterial2d<ColorMaterial>>().is_some() {
                entity.remove::<MeshMaterial2d<ColorMaterial>>();
            } else {
                let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
                entity.insert(MeshMaterial2d(materials.add(color)));
            }
        }
    }
}
