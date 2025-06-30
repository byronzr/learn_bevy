use bevy::{
    input_focus::{
        InputFocus,
        tab_navigation::{TabGroup, TabIndex},
    },
    prelude::*,
};
use bevy_ecs::spawn::{SpawnIter, SpawnWith};

use crate::define::ArgKeyValue;

pub fn text_input_panel(font: Handle<Font>) -> impl Bundle {
    Children::spawn(
        //SpawnWith(|p: &mut RelatedSpawner<ChildOf>| {
        // instead
        // key
        SpawnWith(move |p: &mut ChildSpawner| {
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
            // value
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
            // submit
            p.spawn((
                Button,
                Node {
                    width: Val::Px(50.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    padding: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    column_gap: Val::Px(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderRadius::all(Val::Px(5.0)),
                BackgroundColor(Color::WHITE.with_alpha(0.1)),
                children![(
                    Text::new("Submit"),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ));
        }),
    )
}

pub fn show_arguments_panel(args: &Vec<ArgKeyValue>, font: Handle<Font>) -> impl Bundle {
    Children::spawn(Spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            position_type: PositionType::Relative,
            padding: UiRect::all(Val::Px(5.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            ..default()
        },
        children![(
            Text::new("Arguments"),
            TextFont {
                font: font.clone(),
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )))
}

pub fn arguments_panel(args: &Vec<ArgKeyValue>, font: Handle<Font>, group: i32) -> impl Bundle {
    let font2 = font.clone();
    Children::spawn((
        // show arguments
        SpawnIter(
            args.clone()
                .into_iter()
                .enumerate()
                .map(move |(index, arg)| {
                    //Text::new(arg.key.clone())
                    (
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
                            (
                                // argument flag layout
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
                                )],
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
                                )],
                            ),
                        ],
                        // row background color
                        // BackgroundColor(Color::WHITE.with_alpha(0.1)),
                    )
                }),
        ),
        Spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(5.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            children![(
                // the row on the bottom and inner layout is row
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Relative,
                    padding: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                //TabGroup::new(group),
                TabGroup::modal(),
                text_input_panel(font2.clone()),
            )],
        )),
    ))
}
