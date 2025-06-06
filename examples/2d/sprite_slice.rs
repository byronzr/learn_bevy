//! Showcases sprite 9 slice scaling and tiling features, enabling usage of
//! sprites in multiple resolutions while keeping it in proportion
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn spawn_sprites(
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    mut position: Vec3,
    slice_border: f32,
    style: TextFont,
    gap: f32,
) {
    let cases = [
        // Reference sprite
        // ** 原始比例
        (
            "Original",
            style.clone(),
            Vec2::splat(100.0),
            SpriteImageMode::Auto,
        ),
        // Scaled regular sprite
        // ** Y轴拉伸
        (
            "Stretched",
            style.clone(),
            Vec2::new(100.0, 200.0),
            SpriteImageMode::Auto,
        ),
        // Stretched Scaled sliced sprite
        // ** 分离外边框,拉伸中间部分
        (
            "With Slicing",
            style.clone(),
            Vec2::new(100.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(slice_border),
                center_scale_mode: SliceScaleMode::Stretch,
                ..default()
            }),
        ),
        // Scaled sliced sprite
        // ** 分离外边框后就会出现
        // ** 4个角与4个边
        // ** 当是个边不是正方形时,会出现拉伸
        (
            "With Tiling",
            style.clone(),
            Vec2::new(100.0, 200.0),
            //Vec2::new(512.0, 1024.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.5 },
                // ** 四条边会有平铺方向,上下为横向平铺,左右为纵向平铺
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                ..default()
            }),
        ),
        // Scaled sliced sprite horizontally
        (
            "With Tiling",
            style.clone(),
            Vec2::new(300.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.3 },
                ..default()
            }),
        ),
        // Scaled sliced sprite horizontally with max scale
        (
            "With Corners Constrained",
            style,
            Vec2::new(300.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::square(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.1 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                // ** 四个角的缩放会影响四条边的宽度,阈值为 0.0-1.0
                max_corner_scale: 0.2,
            }),
        ),
    ];

    for (label, text_style, size, scale_mode) in cases {
        position.x += 0.5 * size.x;
        let mut cmd = commands.spawn((
            Sprite {
                image: texture_handle.clone(),
                custom_size: Some(size),
                image_mode: scale_mode,
                ..default()
            },
            Transform::from_translation(position),
        ));
        cmd.with_children(|builder| {
            builder.spawn((
                Text2d::new(label),
                text_style,
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_xyz(0., -0.5 * size.y - 10., 0.0),
                bevy::sprite::Anchor::TopCenter,
            ));
        });
        position.x += 0.5 * size.x + gap;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let style = TextFont {
        font: font.clone(),
        ..default()
    };

    // Load textures
    // 都是 512 * 512 的图片
    //let handle_1 = asset_server.load("textures/slice_square.png");
    let handle_2 = asset_server.load("textures/slice_square_2.png");

    // spawn_sprites(
    //     &mut commands,
    //     handle_1,
    //     Vec3::new(-600.0, 200.0, 0.0),
    //     200.0,
    //     style.clone(),
    //     40.,
    // );

    spawn_sprites(
        &mut commands,
        handle_2,
        Vec3::new(-600.0, -200.0, 0.0),
        80.0,
        style,
        40.,
    );
}
