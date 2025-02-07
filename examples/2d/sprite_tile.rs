//! Displays a single [`Sprite`] tiled in a grid, with a scaling animation

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run();
}

#[derive(Resource)]
struct AnimationState {
    min: f32,
    max: f32,
    current: f32,
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(AnimationState {
        min: 128.0,
        max: 512.0,
        current: 128.0,
        speed: 50.0,
    });
    commands.spawn(Sprite {
        // logo 是圆形背景 256 * 256
        image: asset_server.load("branding/icon.png"),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true, // 如果为 false,则不会在 x 轴上平铺,会在x轴上拉伸
            tile_y: true,
            stretch_value: 0.5, // The image will tile every 128px
        },
        ..default()
    });
}

fn animate(mut sprites: Query<&mut Sprite>, mut state: ResMut<AnimationState>, time: Res<Time>) {
    // 当前大小超过最大或最小值时,反转速度
    if state.current >= state.max || state.current <= state.min {
        state.speed = -state.speed;
    };

    // 正弦方法更简练
    // state.speed = ops::sin(time.elapsed_secs()) * 50.0;

    state.current += state.speed * time.delta_secs();
    for mut sprite in &mut sprites {
        // splat 一个相同分量的 Vec2 (正方形)
        sprite.custom_size = Some(Vec2::splat(state.current));
    }
}
