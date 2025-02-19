//! Simple text input support
//!
//! Return creates a new line, backspace removes the last character.
//! Clicking toggle IME (Input Method Editor) support, but the font used as limited support of characters.
//! You should change the provided font with another one to test other languages input.

use std::mem;

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                toggle_ime,
                listen_ime_events,
                listen_keyboard_input_events,
                bubbling_text,
            ),
        )
        .run();
}

// * 创建基本文本布局
// * 注意 TextSpan 与 Text::default() 占用的 index
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // The default font has a limited number of glyphs, so use the full version for
    // sections that will hold text input.
    // SourceHanSansCN-Normal.otf
    // let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let font = asset_server.load("fonts/SourceHanSansCN-Normal.otf");

    commands
        .spawn((
            Text::default(), // ! index 0
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(TextSpan::new(
                "Click to toggle IME. Press return to start a new line.\n\n", // index 1
            ));
            p.spawn(TextSpan::new("IME Enabled: ")); // index 2
            p.spawn(TextSpan::new("false\n")); // index 3
            p.spawn(TextSpan::new("IME Active:  ")); // index 4
            p.spawn(TextSpan::new("false\n")); // index 5
            p.spawn(TextSpan::new("IME Buffer:  ")); // index 6
            p.spawn((TextSpan::new("\n"),)); // index 7
        });

    commands.spawn((
        Text2d::new(""),
        TextFont {
            font,
            font_size: 100.0,
            ..default()
        },
    ));
}

/// * 输入法开关
fn toggle_ime(
    input: Res<ButtonInput<MouseButton>>,
    mut window: Single<&mut Window>,
    status_text: Single<Entity, (With<Node>, With<Text>)>,
    // * TextUiWriter 是一个配合 root /  node / text / TextSpan 的写入器
    mut ui_writer: TextUiWriter,
) {
    if input.just_pressed(MouseButton::Left) {
        window.ime_position = window.cursor_position().unwrap();
        window.ime_enabled = !window.ime_enabled;

        *ui_writer.text(*status_text, 3) = format!("{} !\n", window.ime_enabled);
    }
}

#[derive(Component)]
struct Bubble {
    timer: Timer,
}

/// * 上浮的 Bubble
/// * 清理超时的 Bubble
fn bubbling_text(
    mut commands: Commands,
    mut bubbles: Query<(Entity, &mut Transform, &mut Bubble)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bubble) in bubbles.iter_mut() {
        if bubble.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
        transform.translation.y += time.delta_secs() * 100.0;
    }
}

/// * 输入法事件监听
fn listen_ime_events(
    mut events: EventReader<Ime>, // * 输入法事件
    status_text: Single<Entity, (With<Node>, With<Text>)>,
    mut edit_text: Single<&mut Text2d, (Without<Node>, Without<Bubble>)>,
    mut ui_writer: TextUiWriter,
) {
    for event in events.read() {
        match event {
            //* Preedit 事件,在 window.im_enabled 开启后才会触发接收
            //* pre-edit 预编辑,在很多非英文输入法中,可能都需要组合
            //* value 为当前输入的值, cursor 为输入游标, 跟鼠标指针无关
            //* 在中文输入法(我用五笔)的操作中,四码上屏与撤码,都会导致 cursor.is_none()
            //* window 在这里几乎不进行处理 ... 可能出现多分离窗体的整合应用才需要.
            Ime::Preedit { value, cursor, .. } if !cursor.is_none() => {
                info!("Preedit cursor not None: {:?}", cursor);
                *ui_writer.text(*status_text, 7) = format!("{value}\n");
            }
            Ime::Preedit { cursor, .. } if cursor.is_none() => {
                warn!("Preedit cursor None!!");
                *ui_writer.text(*status_text, 7) = "\n".to_string();
            }
            //* 只有真正的完成编辑上屏才会得到该通知,同样需要 window.im_enabled 开启
            Ime::Commit { value, .. } => {
                edit_text.push_str(value);
            }
            //* 输入法在切换过程中得到的通知
            Ime::Enabled { .. } => {
                *ui_writer.text(*status_text, 5) = "true\n".to_string();
            }
            Ime::Disabled { .. } => {
                *ui_writer.text(*status_text, 5) = "false\n".to_string();
            }
            _ => (),
        }
    }
}

/// * Enter 上浮输入文本
fn listen_keyboard_input_events(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    edit_text: Single<(&mut Text2d, &TextFont), (Without<Node>, Without<Bubble>)>,
) {
    let (mut text, style) = edit_text.into_inner();
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            // * 回车键触发上浮文本
            Key::Enter => {
                if text.is_empty() {
                    continue;
                }
                // * mem::take 取出原值,并将可写引用的对象重置为 default()
                // * 简洁清晰,避免克隆
                let old_value = mem::take(&mut **text);

                // * 创建一个 Bubble,交给其它 system 进行位移与回收
                commands.spawn((
                    Text2d::new(old_value),
                    style.clone(),
                    Bubble {
                        timer: Timer::from_seconds(5.0, TimerMode::Once),
                    },
                ));
            }
            // * 以下为普通的键盘输入的处理方式,
            // * 只有在 window.im_enabled 关闭的情况下才会触发
            Key::Space => {
                println!("space .");
                text.push(' ');
            }
            Key::Backspace => {
                println!("backspace .");
                text.pop();
            }
            Key::Character(character) => {
                text.push_str(character);
            }
            _ => continue,
        }
    }
}
