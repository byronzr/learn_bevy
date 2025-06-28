use crate::define::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use tokio::sync::{broadcast, mpsc};

use crate::{FONT_BYTES, define::*, ui::ui_menu_button};

// initialize
pub fn setup(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    commands.spawn(Camera2d);

    let font = Font::try_from_bytes(FONT_BYTES.to_vec()).unwrap();
    let font_handle = fonts.add(font);
    commands.insert_resource(FontHandle(font_handle.clone()));

    // layout
    let layout_id = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            //BackgroundColor(Color::srgb_u8(30, 30, 30)),
        ))
        .id();

    // menu
    let menu_id = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                // PositionType::Absolute make width uncontrolled,but position always relative to parent
                // PositionType::Relative make width controlled by parent,margin and padding will affect width
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Start,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(50, 50, 50)),
            children![
                ui_menu_button(MenuImportButton::default(), font_handle.clone()),
                ui_menu_button(MenuSaveButton::default(), font_handle.clone()),
                ui_menu_button(MenuClearButton::default(), font_handle.clone()),
                ui_menu_button(MenuHideButton::default(), font_handle.clone()),
                ui_menu_button(MenuExitButton::default(), font_handle.clone()),
            ],
        ))
        .id();
    commands.entity(layout_id).add_child(menu_id);

    // ui container
    let container_id = commands
        .spawn((
            Container,
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
        ))
        .id();
    commands.entity(layout_id).add_child(container_id);

    // preivew window
    let preview_id = commands
        .spawn((
            PreviewWindow,
            Node {
                width: Val::Percent(25.0),
                height: Val::Percent(25.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                display: Display::Block,
                ..default()
            },
            Visibility::Hidden,
            BackgroundColor(Color::srgb_u8(40, 40, 40)),
        ))
        .id();
    commands.entity(container_id).add_child(preview_id);

    let (progress_tx, progress_rx) = mpsc::channel::<ProgressInfo>(100);
    let (main_tx, _) = broadcast::channel::<ProcessSignal>(100);
    let progress = HashMap::<usize, ProgressStatistics>::new();

    commands.insert_resource(ProcessState {
        progress_tx,
        progress_rx,
        main_tx,
        progress,
    });

    commands.insert_resource(ProcessMenu {
        lock_import: false,
        hide_done: false,
    });
}

pub fn on_window_close(
    mut close_events: EventReader<bevy::window::WindowCloseRequested>,
    process_state: Res<ProcessState>,
) -> Result {
    for _event in close_events.read() {
        info!("interrupt ffmpeg process by window close event");
        let _ = process_state.main_tx.send(ProcessSignal::WindowClose);
    }
    Ok(())
}
