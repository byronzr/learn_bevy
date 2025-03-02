//! 展示了 Camera 与 RenderLayers 的使用
//! TargetCamera 只会作用于 Node, 无法作用于 Sprite
//! Sprite 在使用 RanderLayers 时, 嵌套节点不会继承
//! 当前版本 0.15.3 ,UI 默认依赖于 RenderLaysers::layer(0), 一但变更所有相关操作将无法预测
//! 本例中添加了三个 camera ,两个非 UI camera, 一个 UI camera(layer(0))
//! 按 Space 键切换 camera 观察

use bevy::{
    asset::RenderAssetUsages,
    color::Color,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    utils::HashSet,
    winit::WinitSettings,
};

#[derive(Debug, Component)]
pub struct CameraIndex(pub usize);

#[derive(Debug, Resource, Default)]
pub struct ActiveCamera(pub HashSet<usize>);

#[derive(Debug, Component)]
pub struct CameraName(String);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(WinitSettings::desktop_app());
    app.init_resource::<ActiveCamera>();
    app.add_systems(Startup, setup);
    app.add_systems(Update, (switch, active_camera));

    app.run();
    //app.update();
}

fn active_camera(
    active_camera: Res<ActiveCamera>,
    mut query: Query<(
        &CameraIndex,
        &mut Camera,
        Option<&RenderLayers>,
        &CameraName,
    )>,
) {
    for (idx, mut camera, layer, name) in &mut query {
        if active_camera.0.contains(&idx.0) {
            if !camera.is_active {
                camera.is_active = true;
                println!(
                    "current_camera: {}  / {} / render_layer: {:?}",
                    idx.0, name.0, layer
                );
            }
        } else {
            camera.is_active = false;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut asset_images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // * camera texture /////////////////////////////////////////////////////////////////////////////
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };
    let mut image_container = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image_container.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = asset_images.add(image_container);

    let texture_camera = commands
        .spawn((
            Camera2d,
            CameraName("texture_camera".to_string()),
            // 指定渲染层级 (没必要,也无法预测)
            // RenderLayers::layer(9),
            Camera {
                // 是不是激活都不重要,什么都不会影响
                // is_active: false,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn(Sprite::from_image(asset_server.load("items/AK47.png")));
            parent.spawn(Sprite::from_image(asset_server.load("items/knife.png")));
            parent.spawn(Sprite::from_image(asset_server.load("items/paper.png")));
        });
    // ! 如果不是绘制 3D 纹里,不需要加入 Material 中,image_handle 已经可以在 2D 场景中使用了

    // * camera 1 /////////////////////////////////////////////////////////////////////////////
    // * 在第 8 层的非 UI camera
    commands.spawn((
        Camera2d,
        CameraIndex(1),
        CameraName("NOT UI".to_string()),
        // ! 指定渲染层级
        RenderLayers::layer(8),
        Camera {
            is_active: false,
            ..Default::default()
        },
    ));
    // * 向第 8 层的,第一个非 UI camera 添加一个节点
    // * 该节点红色,有嵌套节点黄色
    commands
        .spawn((
            Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::splat(100.0)),
            // 对 Sprite (非UI) 无效果
            // TargetCamera(camera_1),
            RenderLayers::layer(8),
            Transform::from_xyz(200., 200., 0.),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_color(Color::srgb(1., 1., 0.), Vec2::splat(5.0)),
                // 使用 RenderLayers 后,并不能保证嵌套的有效性,如果不用指定与 Camera 相同的层级,就会写入 0 层
                // 保持与父级一致,视觉基本一致
                RenderLayers::layer(8),
                //  会写入 camera_2 的层级,但却使用的是相对父级的坐标
                // RenderLayers::layer(9),
            ));
        });
    ///////////////////////////////////////////////////////////////////////////////////////////

    // * camera 2 /////////////////////////////////////////////////////////////////////////////
    // * 在第 9 层的非 UI camera
    commands.spawn((
        Camera2d,
        CameraIndex(2),
        CameraName("NOT UI".to_string()),
        RenderLayers::layer(9),
        Camera {
            is_active: false,
            ..Default::default()
        },
    ));

    // * 向第 9 层的,第一个非 UI camera 添加一个节点
    // * 该节点蓝色
    commands.spawn((
        Sprite::from_image(image_handle),
        // ! 对 Sprite (非UI) 无效果
        // TargetCamera(camera_1),
        Transform::from_xyz(-200., -200., 0.),
        RenderLayers::layer(9),
    ));
    ///////////////////////////////////////////////////////////////////////////////////////////

    // * ui camera 0
    // * 在第 0 层,也只能在第 0 层
    let camera_ui_0 = commands
        .spawn((
            Camera2d,
            //IsDefaultUiCamera,
            CameraName(">>> UI".to_string()),
            // ! 除了 0 层,其它层级的 UI camera 都设混乱不可预测并且错误.
            // ! 官方文档说, RanderLayers 如果不进行设置,默认值为 0 ,但是在 Query 中,却是 None
            // RenderLayers::layer(10),
            CameraIndex(0),
            Camera {
                is_active: false,
                ..Default::default()
            },
        ))
        .id();

    // * 向 UI camera 添加一个节点
    // * 该节点有嵌套节点
    commands
        .spawn((
            Node {
                // 当 UI 节点的最外层不打算覆盖整个 Window size 时,就会暴露出其它静默的 camera
                width: Val::Px(50.),
                aspect_ratio: Some(1.0),
                // width: Val::Percent(100.),
                // height: Val::Percent(100.),
                // 默认对齐方式是左上角,在这里指定后居中
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0., 1., 0.)),
            TargetCamera(camera_ui_0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(10.),
                    height: Val::Px(10.),
                    //aspect_ratio: Some(1.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0., 1., 1.)),
            ));
        });
}

fn switch(input: Res<ButtonInput<KeyCode>>, mut active_camera: ResMut<ActiveCamera>) {
    if input.just_pressed(KeyCode::Numpad0) {
        if active_camera.0.contains(&0) {
            active_camera.0.remove(&0);
        } else {
            active_camera.0.insert(0);
        }
    }
    if input.just_pressed(KeyCode::Numpad1) {
        if active_camera.0.contains(&1) {
            active_camera.0.remove(&1);
        } else {
            active_camera.0.insert(1);
        }
    }
    if input.just_pressed(KeyCode::Numpad2) {
        if active_camera.0.contains(&2) {
            active_camera.0.remove(&2);
        } else {
            active_camera.0.insert(2);
        }
    }
}
