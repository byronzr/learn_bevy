use bevy::prelude::*;
use bevy::{asset::processor::Process, platform::collections::HashMap};
use tokio::sync::{broadcast, mpsc};

use crate::{define::*, ui::ui_menu_button};

// initialize
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

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
                ui_menu_button("Lock Import".to_string(), &asset_server,),
                ui_menu_button("Save Status".to_string(), &asset_server,),
                ui_menu_button("Clear".to_string(), &asset_server,),
                ui_menu_button("Hide Done".to_string(), &asset_server,),
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
        println!("interrupt ffmpeg process by window close event");
        process_state.main_tx.send(ProcessSignal::WindowClose)?;
    }
    Ok(())
}
