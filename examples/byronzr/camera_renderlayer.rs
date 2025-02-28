//! 展示了 Camera 与 RenderLayers 的使用
//! TargetCamera 只会作用于 Node, 无法作用于 Sprite
//! Sprite 在使用 RanderLayers 时, 嵌套节点不会继承
//! 当前版本 0.15.3 ,UI 默认依赖于 RenderLaysers::layer(0), 一但变更所有相关操作将无法预测
//! 本例中添加了三个 camera ,两个非 UI camera, 一个 UI camera(layer(0))
//! 按 Space 键切换 camera 观察

use bevy::{color::Color, prelude::*, render::view::RenderLayers, utils::HashMap};

#[derive(Debug, Component)]
pub struct CameraIndex(pub usize);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, switch);

    app.run();
    //app.update();
}

fn setup(mut commands: Commands) {
    // * camera 0
    // * 在第 8 层的非 UI camera
    commands.spawn((
        Camera2d,
        CameraIndex(0),
        // ! 指定渲染层级
        RenderLayers::layer(8),
        Camera {
            is_active: true,
            ..Default::default()
        },
    ));

    // * camera 1
    // * 在第 9 层的非 UI camera
    commands.spawn((
        Camera2d,
        CameraIndex(1),
        RenderLayers::layer(9),
        Camera {
            is_active: false,
            ..Default::default()
        },
    ));

    // * ui camera 0
    // * 在第 0 层,也只能在第 0 层
    let camera_ui_0 = commands
        .spawn((
            Camera2d,
            IsDefaultUiCamera,
            // ! 除了 0 层,其它层级的 UI camera 都设混乱不可预测并且错误.
            // RenderLayers::layer(10),
            CameraIndex(2),
            Camera {
                is_active: false,
                ..Default::default()
            },
        ))
        .id();

    // * 向第 8 层的,第一个非 UI camera 添加一个节点
    // * 该节点红色,有嵌套节点黄色
    commands
        .spawn((
            Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::splat(100.0)),
            // ! 对 Sprite (非UI) 无效果
            // TargetCamera(camera_1),
            RenderLayers::layer(8),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_color(Color::srgb(1., 1., 0.), Vec2::splat(5.0)),
                // ! 在非UI节点上的嵌套节点不会被应用到父级相同的 RenderLayers
                // * 这大概率是一个 BUG ,在这个例子中,被渲然到 0 层
                RenderLayers::layer(8),
            ));
        });

    // * 向第 9 层的,第一个非 UI camera 添加一个节点
    // * 该节点蓝色
    commands.spawn((
        Sprite::from_color(Color::srgb(0., 0., 1.), Vec2::splat(100.0)),
        // ! 对 Sprite (非UI) 无效果
        // TargetCamera(camera_0),
        RenderLayers::layer(9),
    ));

    // * 向 UI camera 添加一个节点
    // * 该节点有嵌套节点
    commands
        .spawn((
            Node {
                width: Val::Px(50.),
                aspect_ratio: Some(1.0),
                // ! 默认对齐方式是左上角,在这里指定后居中
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

fn switch(mut query: Query<(&mut Camera, &CameraIndex)>, input: Res<ButtonInput<KeyCode>>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let mut current_layer = 0;
    let mut cameras = HashMap::new();
    let camera_count = query.iter().len();
    for (mut camera, idx) in &mut query {
        let layer = idx.0;
        if camera.is_active {
            current_layer = layer + 1;
            if current_layer >= camera_count {
                current_layer = 0;
            }
        }
        camera.is_active = false;
        cameras.insert(layer, camera);
    }
    println!("current_layer: {} ", current_layer);
    cameras.get_mut(&current_layer).unwrap().is_active = true;
}
