use bevy::{
    input_focus::{InputFocus, tab_navigation::TabIndex},
    prelude::*,
};
use bevy_ecs::{relationship::RelatedSpawner, spawn::SpawnWith};

use crate::define::ArgKeyValue;

pub fn text_input_ex() -> impl Bundle {
    Children::spawn(SpawnWith(|p: &mut RelatedSpawner<ChildOf>| {
        p.spawn((
            Node {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(5.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            BorderRadius::all(Val::Px(5.0)),
            BackgroundColor(Color::WHITE.with_alpha(0.1)),
            TabIndex(0),
        ))
        .observe(
            |mut trigger: Trigger<Pointer<Click>>, mut focus: ResMut<InputFocus>| {
                focus.0 = Some(trigger.target());
                trigger.propagate(false);
            },
        );
    }))
}

pub fn text_input(parent: &mut ChildSpawnerCommands<'_>) {
    parent
        .spawn((
            Node {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(5.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            BorderRadius::all(Val::Px(5.0)),
            BackgroundColor(Color::WHITE.with_alpha(0.1)),
            TabIndex(0),
        ))
        .observe(
            |mut trigger: Trigger<Pointer<Click>>, mut focus: ResMut<InputFocus>| {
                focus.0 = Some(trigger.target());
                trigger.propagate(false);
            },
        );
}

pub fn show_arguments(
    parent: &mut ChildSpawnerCommands<'_>,
    args: &Vec<ArgKeyValue>,
    font: Handle<Font>,
) {
    for (index, arg) in args.iter().enumerate() {
        // argument one row layout
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                position_type: PositionType::Relative,
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                // argument flag layout
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        position_type: PositionType::Relative,
                        padding: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(5.0)),
                    BackgroundColor(Color::srgb_u8(0, 0, 255).with_alpha(0.1)),
                    // the text of flag
                    children![(
                        Text::new(arg.key.clone()),
                        TextFont {
                            font: font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )]
                ),
                // argument value layout
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        position_type: PositionType::Relative,
                        padding: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(5.0)),
                    BackgroundColor(Color::WHITE.with_alpha(0.1)),
                    // the text of value
                    children![(
                        Text::new(arg.value.clone()),
                        TextFont {
                            font: font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )]
                ),
                //SpawnWith(|parent: &mut ChildSpawner| { parent.spawn(()) }),
            ],
        ));
    }
}
