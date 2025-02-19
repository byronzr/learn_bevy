//! Demonstrates how to grab and hide the mouse cursor.
// * 鼠标截取(隐藏)
use bevy::{prelude::*, window::CursorGrabMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, grab_mouse)
        .run();
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    // * mouse 的可视状态
    if mouse.just_pressed(MouseButton::Left) {
        // * 隐藏鼠标
        window.cursor_options.visible = true;
        // * 锁定鼠标
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
