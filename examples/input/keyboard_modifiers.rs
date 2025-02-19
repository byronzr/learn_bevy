//! Demonstrates using key modifiers (ctrl, shift).
//* 修饰键(控制键,shift键)的使用与组合用法

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

/// This system prints when `Ctrl + Shift + A` is pressed
/// * 注意: 从 Resource 中读取 ButtonInput,而不是事件
fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
        info!("Just pressed Ctrl + Shift + A!");
    }
}
