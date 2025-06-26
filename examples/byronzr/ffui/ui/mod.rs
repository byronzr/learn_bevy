use crate::define::*;
use bevy::prelude::*;
pub mod interaction;
pub mod refresh;
pub mod setup;

pub use interaction::*;
pub use refresh::*;

pub fn ui_task_button(index: usize, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        IndexOfline(index),
        TaskButton,
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
        BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("convert"),
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
        BorderColor(Color::WHITE.with_alpha(0.2)),
        BackgroundColor(Color::srgb_u8(0, 0, 0)),
        children![(
            Text::new("replace"),
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
        BorderColor(Color::WHITE.with_alpha(0.2)),
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

pub fn ui_menu_button(name: String, font: Handle<Font>) -> impl Bundle {
    (
        Button,
        Name::new(name.clone()),
        MenuButton,
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
