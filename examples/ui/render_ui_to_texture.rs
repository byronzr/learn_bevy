//! Shows how to render UI to a texture. Useful for displaying UI in 3D space.

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotator_system)
        .run();
}

// Marks the cube, to which the UI texture is applied.
#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    // ! 定义一个目标容器 (bevy_image)
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // You need to set these texture usage flags in order to use the image as a render target
    // ! 成为一个渲染目标,需要设置这些标志
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    // ! 将 image 加入到 image 的资源库中 (Assets<Image>)
    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLight::default());

    // ! 这里,得到一个离屏渲染的 camera,相当于一个投射转换器
    let texture_camera = commands
        .spawn((
            Camera2d,
            // ! 渲染目标是一个图像,意味着该图像可以使用到其它地方,真正的 Camera 是一个 Camera3d
            // ! 因为是一个纹理,并没有要求被 camera 显示,所以可以得到一个离屏渲染的机制
            Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
        ))
        .id();

    // ! 这个 Entity 被创建在 texture_camera 投射中,
    // ! 经过这一部操作 image_handle 才有了,实际的纹理数据,
    // ! image_handle 才可以被使用
    commands
        .spawn((
            Node {
                // Cover the whole image
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(GOLD.into()),
            // ! 指定这个 node 使用 camera
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            // ! 纹理中使用图片
            parent.spawn((ImageNode::new(
                asset_server.load("branding/bevy_logo_dark_big.png"),
            ),));
            // ! 纹理中使用文字
            parent.spawn((
                Text::new("This is a cube"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor::BLACK,
            ));
        });

    // This material has the texture that has been rendered.
    // ! 将 image_handle 加入材质库
    // ! 至此完成了关于一个纹理的创建的全部过程
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // ! 开始使用该纹理
    let cube_size = 4.0;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // Cube with material containing the rendered UI texture.
    commands.spawn((
        Mesh3d(cube_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        Cube,
    ));

    // The main pass camera.
    // ! 主视窗的 camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

const ROTATION_SPEED: f32 = 0.5;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_secs() * ROTATION_SPEED);
        transform.rotate_y(0.7 * time.delta_secs() * ROTATION_SPEED);
    }
}
