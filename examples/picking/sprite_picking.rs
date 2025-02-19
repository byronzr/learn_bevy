//! Demonstrates picking for sprites and sprite atlases. The picking backend only tests against the
//! sprite bounds, so the sprite atlas can be picked by clicking on its transparent areas.

use bevy::{prelude::*, sprite::Anchor, winit::WinitSettings};
use std::fmt::Debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // ! 增加 desktop_app 有效改善鼠标事件的响应
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup, setup_atlas))
        .add_systems(Update, (move_sprite, animate_sprite))
        .run();
}

// * 缓慢移动
fn move_sprite(
    time: Res<Time>,
    mut sprite: Query<&mut Transform, (Without<Sprite>, With<Children>)>,
) {
    let t = time.elapsed_secs() * 0.1;
    for mut transform in &mut sprite {
        let new = Vec2 {
            x: 50.0 * ops::sin(t),
            y: 50.0 * ops::sin(t * 2.0),
        };
        transform.translation.x = new.x;
        transform.translation.y = new.y;
        // ! 保持 Z 轴不变
        transform.translation.z = 0.0;
    }
}

/// Set up a scene that tests all sprite anchor types.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let len = 128.0;
    let sprite_size = Vec2::splat(len / 2.0);

    commands
        .spawn((Transform::default(), Visibility::default()))
        .with_children(|commands| {
            for (anchor_index, anchor) in [
                Anchor::TopLeft,
                Anchor::TopCenter,
                Anchor::TopRight,
                Anchor::CenterLeft,
                Anchor::Center,
                Anchor::CenterRight,
                Anchor::BottomLeft,
                Anchor::BottomCenter,
                Anchor::BottomRight,
                Anchor::Custom(Vec2::new(0.5, 0.5)),
                Anchor::Custom(Vec2::new(0.5, 0.5)),
            ]
            .iter()
            .enumerate()
            {
                // *  3x3 矩阵 + 1 自定义
                // * 注意: 坐标系原点在中心,所以图形绘制过程是从左下到右上
                // * 最后一个自定义正好在左上角
                let i = (anchor_index % 3) as f32;
                let j = (anchor_index / 3) as f32;

                // spawn black square behind sprite to show anchor point
                // * 创建黑色锚点方块
                commands
                    .spawn((
                        Sprite::from_color(Color::BLACK, sprite_size),
                        Transform::from_xyz(i * len - len, j * len - len, -1.0),
                    ))
                    // ! Drag 事件才更加有意义
                    //.observe(custom_drag_start::<Pointer<DragStart>>())
                    .observe(custom_drag_start::<Pointer<Drag>>())
                    .observe(custom_drag_start::<Pointer<DragEnd>>())
                    .observe(custom_drag_start::<Pointer<DragDrop>>())
                    // * 经过时蓝色
                    .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
                    // * 离开后黑色
                    .observe(recolor_on::<Pointer<Out>>(Color::BLACK))
                    // * 按下时黄色
                    .observe(recolor_on::<Pointer<Down>>(Color::srgb(1.0, 1.0, 0.0)))
                    // * 松开后蓝色
                    // ! (原例子中,使用经过时的青色,看不出效果,很槽糕)
                    //.observe(recolor_on::<Pointer<Up>>(Color::srgb(0.0, 1.0, 1.0)));
                    .observe(recolor_on::<Pointer<Up>>(Color::srgb(0.0, 0.0, 1.0)));

                // * 创建 logo 并改变锚点
                // ! 保证有两个无遮挡的 矩形进行叠加尝试
                if anchor_index < 8 {
                    commands
                        .spawn((
                            Sprite {
                                image: asset_server.load("branding/bevy_bird_dark.png"),
                                custom_size: Some(sprite_size),
                                color: Color::srgb(1.0, 0.0, 0.0),
                                // * 在这里给 Sprite 设置锚点
                                anchor: anchor.to_owned(),
                                ..default()
                            },
                            // 3x3 grid of anchor examples by changing transform
                            Transform::from_xyz(i * len - len, j * len - len, 0.0)
                                .with_scale(Vec3::splat(1.0 + (i - 1.0) * 0.2))
                                // ! 展示了旋转受到锚点影响
                                .with_rotation(Quat::from_rotation_z((j - 1.0) * 0.2)),
                        ))
                        // ! 同样,带图的 Sprite 对于鼠标事件的响应
                        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 0.0)))
                        .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 0.0, 0.0)))
                        .observe(recolor_on::<Pointer<Down>>(Color::srgb(0.0, 0.0, 1.0)))
                        .observe(recolor_on::<Pointer<Up>>(Color::srgb(0.0, 1.0, 0.0)));
                }
            }
        });
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        timer.tick(time.delta());

        if timer.just_finished() {
            texture_atlas.index = if texture_atlas.index == indices.last {
                indices.first
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

fn setup_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 7, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands
        .spawn((
            Sprite::from_atlas_image(
                texture_handle,
                TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index: animation_indices.first,
                },
            ),
            Transform::from_xyz(300.0, 0.0, 0.0).with_scale(Vec3::splat(6.0)),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Down>>(Color::srgb(1.0, 1.0, 0.0)))
        .observe(recolor_on::<Pointer<Up>>(Color::srgb(0.0, 1.0, 1.0)));
}

// An observer listener that changes the target entity's color.
// * Pointer<Over>, Pointer 是内置在 bevy_picking 中的类型,
// * 能够处理的事件还挺多的,
// * 这里只处理 Over, Out, Down, Up
// @ Click 当一个指针的按下与释放在同一个目标实体上是触发
// @ Move 事件与 Over 事件类似
fn recolor_on<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = color;
    }
}

// ! 这此功能对于进行物品栏摆放时非常有用
// @ DragStart 拖拽开始事件
// @ DragEnd 拖拽结束事件
// @ DragEnter .dragged 实体进入 hit.entity 实体时触发
// @ DragOver .dragged 实体在 hit.entity 实体上移动时触发
// @ DragLeave .dragged 实体离开 hit.entity 实体时触发
// @ DragDrop .dragged 实体在 hit.entity 实体上释放时触发
fn custom_drag_start<E: Debug + Clone + Reflect>(
) -> impl Fn(Trigger<E>, Query<(&mut Sprite, &mut Transform)>) {
    move |ev, mut sprites| {
        let Ok((mut sprite, mut transform)) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = Color::WHITE;

        let Some(reflect) = ev.event().try_as_reflect() else {
            return;
        };

        // * 无叠加对象结束
        if let Some(v) = reflect.downcast_ref::<Pointer<DragEnd>>() {
            println!("DragEnd: {:?}", v.distance);
        }

        // * 拖拽对象

        if let Some(Pointer { event, .. }) = reflect.downcast_ref::<Pointer<Drag>>() {
            // ! 应用原 sprite 的 Z轴信息
            let z = transform.translation.z;
            // ! 注意: 这里的 delta 是相对于上一帧的位移向量,所以 y 轴是反的
            transform.translation += (event.delta * Vec2::new(1., -1.)).extend(0.);
            // ! 恢复 Z 轴不变,
            transform.translation.z = z;
        }

        // * 有叠加对象结束
        // ! 注意: 所谓的叠加状态,是可以从颜色上区别的
        // ! 并且,拖拽对象与叠加对象都要被 observe 相同的 Pointer<DragDrop>
        if let Some(v) = reflect.downcast_ref::<Pointer<DragDrop>>() {
            println!(
                "DragDrop: drop entity({:?}) ==> target({:?})",
                v.dropped,
                ev.entity()
            );
        }
    }
}
