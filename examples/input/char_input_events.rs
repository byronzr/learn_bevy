//! Prints out all chars as they are inputted.
//* 简单的字符输入事件示例

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, print_char_event_system)
        .run();
}

/// This system prints out all char events as they come in.
fn print_char_event_system(mut char_input_events: MessageReader<KeyboardInput>) {
    for event in char_input_events.read() {
        // Only check for characters when the key is pressed.
        // * 确认按键是否按下,才继续读取
        if !event.state.is_pressed() {
            continue;
        }
        if let Key::Character(character) = &event.logical_key {
            info!("{:?}: '{}'", event, character);
        }
    }
}
