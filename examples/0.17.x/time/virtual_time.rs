//! Shows how `Time<Virtual>` can be used to pause, resume, slow down
//! and speed up a game.

use std::time::Duration;

use bevy::{
    color::palettes::css::*, input::common_conditions::input_just_pressed, prelude::*,
    time::common_conditions::on_real_timer,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_virtual_time_sprites, // 变速 Sprite 的 system
                move_real_time_sprites,    // 原速 Sprite 的 system
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
                change_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowUp)), // system,通过泛型常量来传参,减少创建别名函数
                change_time_speed::<0>.run_if(input_just_pressed(KeyCode::ArrowDown)), // 官方案例中是 -1,但我的连体字会影响显示
                (update_virtual_time_info_text, update_real_time_info_text)
                    // update the texts on a timer to make them more readable
                    // `on_timer` run condition uses `Virtual` time meaning it's scaled
                    // and would result in the UI updating at different intervals based
                    // on `Time<Virtual>::relative_speed` and `Time<Virtual>::is_paused()`
                    // 此处理的条件控制控制界面(UI)文本更新(密度),如果"密度"过高,阅读的意义就不大了
                    .run_if(on_real_timer(Duration::from_millis(250))),
            ),
        )
        .run();
}

/// `Real` time related marker
/// 这个 Component 结构,会用在两个地方,
/// 一个是 Sprite ,方便 Query 查询
/// 一个是 UI Text
#[derive(Component)]
struct RealTime;

/// `Virtual` time related marker
#[derive(Component)]
struct VirtualTime;

/// Setup the example
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Virtual>>) {
    // start with double `Virtual` time resulting in one of the sprites moving at twice the speed
    // of the other sprite which moves based on `Real` (unscaled) time
    // 初始变速资源倍率为2倍
    time.set_relative_speed(2.);

    // 添加一个 2D 摄像机
    commands.spawn(Camera2d);

    // 变速图标的底色(金色)
    let virtual_color = GOLD.into();
    let sprite_scale = Vec2::splat(0.5) // 初始化一个二维矢量
        .extend(1.); // 扩展一个二维矢量至三维
    let texture_handle = asset_server.load("branding/icon.png"); // 读取资产库中的图片

    // the sprite moving based on real time
    // 创建一个原速(Time<Real>)实体(entity)的外观与形变
    commands.spawn((
        Sprite::from_image(texture_handle.clone()),
        Transform::from_scale(sprite_scale),
        RealTime,
    ));

    // the sprite moving based on virtual time
    // 创建一个原速(Time<Virtual>)实体(entity)的外观与形变
    commands.spawn((
        Sprite {
            image: texture_handle,
            color: virtual_color, // 区别原速 Sprite 的底色
            ..Default::default()
        },
        Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., -160., 0.), // 让变速 Sprite 不与原速 Sprite 动画同轨,所以偏移至中心点y轴向下
            ..default()
        },
        VirtualTime,
    ));

    // info UI
    let font_size = 33.;

    commands
        // UI 层级父节点(Root)
        .spawn(Node {
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        // 三列信息
        .with_children(|builder| {
            // real time info
            // 左对齐的原速信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                RealTime,
            ));

            // keybindings
            // 中间的控制提示
            builder.spawn((
                Text::new("CONTROLS\nUn/Pause: Space\nSpeed+: Up\nSpeed-: Down"),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextLayout::new_with_justify(Justify::Center),
            ));

            // virtual time info
            // 右对齐的变速信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(virtual_color),
                TextLayout::new_with_justify(Justify::Right),
                VirtualTime,
            ));
        });
}

/// Move sprites using `Real` (unscaled) time
fn move_real_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<RealTime>)>,
    // `Real` time which is not scaled or paused
    // Time<Real> 是 Bevy 中默认就存在不会被加速和暂停的全局资源
    time: Res<Time<Real>>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Real` second
        // when the time is scaled the speed is going to change
        // and the sprite will stay still the time is paused
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

/// Move sprites using `Virtual` (scaled) time
fn move_virtual_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<VirtualTime>)>,
    // the default `Time` is either `Time<Virtual>` in regular systems
    // or `Time<Fixed>` in fixed timestep systems so `Time::delta()`,
    // `Time::elapsed()` will return the appropriate values either way
    // Time 会是 Time<Virutal> 与 Time<Fixed> 中的一种
    time: Res<Time>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Virtual` second
        // when time is scaled using `Time<Virtual>::set_relative_speed` it's going
        // to move at a different pace and the sprite will stay still when time is
        // `Time<Virtual>::is_paused()`
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

// 利用三角函数正弦波形控制位移
fn get_sprite_translation_x(elapsed: f32) -> f32 {
    ops::sin(elapsed) * 500.
}

/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    // 增加,将 0/1 置换为 -1/+1
    let val = if DELTA == 0 { -1_f32 } else { 1_f32 };
    let time_speed = (time.relative_speed() + val)
        .round() // 四舍五入
        .clamp(0.25, 5.); // 钳制最终值(避免出现负的速度与超高速)

    // set the speed of the virtual time to speed it up or slow it down
    time.set_relative_speed(time_speed);
}

/// pause or resume `Relative` time
fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

/// Update the `Real` time info text
/// 更新原速文本信息
fn update_real_time_info_text(time: Res<Time<Real>>, mut query: Query<&mut Text, With<RealTime>>) {
    for mut text in &mut query {
        **text = format!(
            "REAL TIME\nElapsed: {:.1}\nDelta: {:.5}\n",
            time.elapsed_secs(),
            time.delta_secs(),
            // Time<Real> 不存在相对速度倍率的方法
            // time.relative_speed(),
        );
    }
}

/// Update the `Virtual` time info text
/// 更新变速文本信息
fn update_virtual_time_info_text(
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Text, With<VirtualTime>>,
) {
    for mut text in &mut query {
        **text = format!(
            "VIRTUAL TIME\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
            time.elapsed_secs(),
            time.delta_secs(),
            time.relative_speed()
        );
    }
}
