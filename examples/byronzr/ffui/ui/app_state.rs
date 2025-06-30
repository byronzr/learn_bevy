use crate::{define::*, ui::*};
use bevy::{input_focus::InputFocus, prelude::*};

pub fn enter_monitor(
    mut commands: Commands,
    process_state: Res<ProcessState>,
    mut data: ResMut<PathDatas>,
) {
    // ui container
    let Some(layout_id) = process_state.layout else {
        error!("ProcessState layout is not set");
        return;
    };

    let container_id = commands
        .spawn((
            StateScoped(AppState::Monitor),
            LinesContainer,
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                overflow: Overflow::scroll_y(),
                align_self: AlignSelf::Stretch,
                ..default()
            },
        ))
        .id();
    commands.entity(layout_id).add_child(container_id);
}

pub fn enter_setting(
    mut commands: Commands,
    process_state: Res<ProcessState>,
    mut ffmpeg_arg: ResMut<FfmpegArg>,
    font: Res<FontHandle>,
    mut focus: ResMut<InputFocus>,
) {
    // ui container
    let Some(layout_id) = process_state.layout else {
        error!("ProcessState layout is not set");
        return;
    };

    // reset focus
    focus.0 = None;

    let setting_id = commands
        .spawn((
            StateScoped(AppState::Setting),
            SettingContainer,
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                justify_content: JustifyContent::SpaceBetween,
                //align_items: AlignItems::Stretch,
                ..default()
            },
        ))
        .id();

    let ffmpeg_hw_id = commands
        .spawn(
            // ffmpeg hw arguments
            (
                Node {
                    //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    //height: Val::Px(300.0), // or Val::Percent(50.0)
                    position_type: PositionType::Relative,
                    //margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    //align_self: AlignSelf::Stretch,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::BLACK.with_alpha(0.5)),
                arguments_panel(&ffmpeg_arg.hw_convert, font.0.clone(), 0),
            ),
        )
        .id();
    commands.entity(setting_id).add_child(ffmpeg_hw_id);

    // ffmpeg sf arguments
    let ffmpeg_sf_id = commands
        .spawn((
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(1.0)),
                row_gap: Val::Px(5.0),
                align_self: AlignSelf::Stretch,
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.5)),
            arguments_panel(&ffmpeg_arg.sf_convert, font.0.clone(), 1),
            //BackgroundColor(Color::srgb_u8(0, 128, 0)),
        ))
        .id();
    commands.entity(setting_id).add_child(ffmpeg_sf_id);
    // preview arguments
    let preivew_id = commands
        .spawn((
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                align_self: AlignSelf::Stretch,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.5)),
            arguments_panel(&ffmpeg_arg.snapshot, font.0.clone(), 2),
            //BackgroundColor(Color::srgb_u8(0, 0, 128)),
        ))
        .id();
    commands.entity(setting_id).add_child(preivew_id);
    // analyze arguments
    let analyze_id = commands
        .spawn((
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                align_self: AlignSelf::Stretch,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.5)),
            arguments_panel(&ffmpeg_arg.analyze, font.0.clone(), 3),
        ))
        .id();
    commands.entity(setting_id).add_child(analyze_id);

    commands.entity(layout_id).add_child(setting_id);
}

pub fn focus_system(
    mut commands: Commands,
    focus: Res<InputFocus>,
    mut query: Query<Entity, With<Node>>,
) {
    if focus.is_changed() {
        for button in query.iter_mut() {
            if focus.0 == Some(button) {
                commands.entity(button).insert(Outline {
                    color: Color::WHITE,
                    width: Val::Px(2.0),
                    offset: Val::Px(2.0),
                });
            } else {
                commands.entity(button).remove::<Outline>();
            }
        }
    }
}
