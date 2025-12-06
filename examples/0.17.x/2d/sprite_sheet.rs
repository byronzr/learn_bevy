//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// 精灵动画循环系统
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Sprite::from_atlas_image 创建的 Sprite
            if let Some(atlas) = &mut sprite.texture_atlas {
                // 推进桢
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

// 加载与初始化动画精灵(静桢)
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // (7*24) 168 * 24
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    // 自定义 layout 布局网格
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    // 将布局添加到资源管理器中
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    // 定义头尾索引桢 0 桢为站立状态,所以是 1-6
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2d);
    commands.spawn((
        // 从图集中创建精灵
        // from_atlas_image 确保 Sprite.texture_atlas = Some(value);
        Sprite::from_atlas_image(
            texture,
            // 绑定布局与索引(起始)
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        // 自定义动画循环计时器
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
