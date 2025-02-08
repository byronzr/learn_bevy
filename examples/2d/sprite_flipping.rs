//! Displays a single [`Sprite`], created from an image, but flipped on one axis.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: asset_server.load("branding/bevy_bird_dark.png"),
        // Flip the logo to the left
        // ** 以 X 轴为中心翻转
        flip_x: true,
        // And don't flip it upside-down ( the default )
        // ** Y 轴
        flip_y: false,
        ..Default::default()
    });
}
