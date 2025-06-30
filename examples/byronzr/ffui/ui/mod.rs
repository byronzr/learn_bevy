use crate::define::*;
use bevy::prelude::*;

pub mod app_state;
pub mod line_interaction;
pub mod menu_interaction;
pub mod refresh;
pub mod setup;
pub mod ui_text_input;

pub use app_state::*;
pub use line_interaction::*;
pub use menu_interaction::*;
pub use refresh::*;
pub use ui_text_input::*;

use accesskit::{Node as Accessible, Role};
use bevy::a11y::AccessibilityNode;

pub fn ui_task_button(index: usize, font: Handle<Font>) -> impl Bundle {
    (
        AccessibilityNode(Accessible::new(Role::ListItem)),
        Pickable {
            should_block_lower: false,
            ..default()
        },
        Button,
        IndexOfline(index),
        TaskButton,
        TaskButtonType(true),
        Node {
            width: Val::Px(60.),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        //BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("hw"),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}

pub fn ui_task_ex_button(index: usize, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        IndexOfline(index),
        TaskButton,
        TaskButtonType(false),
        Node {
            width: Val::Px(60.),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        //BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("sf"),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}

pub fn ui_replace_button(index: usize, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        IndexOfline(index),
        ReplaceButton,
        Node {
            width: Val::Px(60.),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        //BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("move"),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}

pub fn ui_snap_button(index: usize, font: Handle<Font>, source: bool) -> impl Bundle {
    (
        Button,
        IndexOfline(index),
        SnapshotButton(source),
        Node {
            width: Val::Px(70.),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        //BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new(format!("snap {}", if source { "A" } else { "B" })),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}

pub fn ui_open_button(index: usize, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        IndexOfline(index),
        OpenButton,
        Node {
            width: Val::Px(60.),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        //BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("open"),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}

pub fn ui_menu_button<T: MenuButtonType + MenuButtonNext + std::fmt::Debug>(
    bt: T,
    font: Handle<Font>,
) -> impl Bundle {
    let name = bt.to_string();

    (
        Button,
        Name::new(name.clone()),
        MenuButton {
            button_type: Box::new(bt),
        },
        Node {
            width: Val::Px(160.),
            height: Val::Px(30.0),
            //border: UiRect::all(Val::Px(3.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(3.0)),
        BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(128, 128, 128)),
        children![(
            Text::new(name),
            TextFont {
                font,
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}
