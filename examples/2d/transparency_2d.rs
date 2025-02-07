//! Demonstrates how to use transparency in 2D.
//! Shows 3 bevy logos on top of each other, each with a different amount of transparency.
//!
//! 透明度的使用方法

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let sprite_handle = asset_server.load("branding/icon.png");

    commands.spawn(Sprite::from_image(sprite_handle.clone()));
    commands.spawn((
        Sprite {
            image: sprite_handle.clone(),
            // Alpha channel of the color controls transparency.
            // 在这里展示了 srgba 最后的 a,就是 alpha 参数,阈值为 0.0-1.0
            color: Color::srgba(0.0, 0.0, 1.0, 0.7),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            image: sprite_handle,
            color: Color::srgba(0.0, 1.0, 0.0, 0.3),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}
