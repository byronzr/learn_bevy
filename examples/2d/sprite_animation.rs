//! Animates a sprite in response to a keyboard event.
//!
//! See `sprite_sheet.rs` for an example where the sprite animation loops indefinitely.
//!
//! 计时器(Timer)在 Sprite 动画中的应用

use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

fn main() {
    App::new()
        // 使用 ImagePlugin::default_nearest() 变更默认采样方式,防止模糊的精灵
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                // press the right arrow key to animate the right sprite
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                // press the left arrow key to animate the left sprite
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .run();
}

// This system runs when the user clicks the left arrow key or right arrow key
// ** 重置计时器,让动画重新开始
fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    // we create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // ** 如果到达最后桢,那么回到第一桢
                // ** 但并不重置计时器,所以动画结束
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;
                }
                // ** 如果不是最后桢,那么移动到下一桢
                // ** 并重置计时器(保证下一桢可以执行)
                else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // the first (left-hand) sprite runs at 10 FPS
    let animation_config_1 = AnimationConfig::new(1, 6, 10);

    // create the first (left-hand) sprite
    // ** 左精灵
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_1.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(-50.0, 0.0, 0.0)),
        LeftSprite,
        animation_config_1,
    ));

    // the second (right-hand) sprite runs at 20 FPS
    let animation_config_2 = AnimationConfig::new(1, 6, 20);

    // create the second (right-hand) sprite
    // ** 右精灵
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_2.first_sprite_index,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(50.0, 0.0, 0.0)),
        RightSprite,
        animation_config_2,
    ));

    // create a minimal UI explaining how to interact with the example
    // ** 说明文本
    commands.spawn((
        Text::new("Left Arrow Key: Animate Left Sprite\nRight Arrow Key: Animate Right Sprite"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
