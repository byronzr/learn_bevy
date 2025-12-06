//! Demonstrates how to grab and hide the mouse cursor.
// * 鼠标截取(隐藏)
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, grab_mouse)
        .run();
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
// fn grab_mouse(
//     mut window: Single<&mut Window>,
//     mouse: Res<ButtonInput<MouseButton>>,
//     key: Res<ButtonInput<KeyCode>>,
// ) {
//     // * mouse 的可视状态
//     if mouse.just_pressed(MouseButton::Left) {
//         // * 隐藏鼠标
//         window.cursor_options.visible = true;
//         // * 锁定鼠标
//         window.cursor_options.grab_mode = CursorGrabMode::Locked;
//     }

//     if key.just_pressed(KeyCode::Escape) {
//         window.cursor_options.visible = true;
//         window.cursor_options.grab_mode = CursorGrabMode::None;
//     }
// }

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    // since bevy 0.17.0
    // 可以直接查询和修改资源
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
