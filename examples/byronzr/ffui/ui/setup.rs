use crate::define::*;

use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
};
use tokio::sync::{broadcast, mpsc};

use crate::{FONT_BYTES, ui::ui_menu_button};
const LINE_HEIGHT: f32 = 30.0;

// initialize
pub fn setup(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
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
                ui_menu_button(MenuLoadButton::default(), font_handle.clone()),
                ui_menu_button(MenuClearButton::default(), font_handle.clone()),
                ui_menu_button(MenuHideButton::default(), font_handle.clone()),
                ui_menu_button(MenuToggleSetting::default(), font_handle.clone()),
                ui_menu_button(MenuExitButton::default(), font_handle.clone()),
            ],
        ))
        .id();
    commands.entity(layout_id).add_child(menu_id);

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
            ZIndex(99),
            Visibility::Hidden,
            BackgroundColor(Color::srgb_u8(40, 40, 40)),
        ))
        .id();
    commands.entity(layout_id).add_child(preview_id);

    let (progress_tx, progress_rx) = mpsc::channel::<ProgressInfo>(100);
    let (main_tx, _) = broadcast::channel::<ProcessSignal>(100);
    //let progress = HashMap::<usize, ProgressStatistics>::new();

    commands.insert_resource(ProcessState {
        progress_tx,
        progress_rx,
        main_tx,
        layout: Some(layout_id),
    });

    commands.insert_resource(ProcessMenu {
        import_type: MenuImportButton::Sequence,
        hide_done: false,
        toggle_setting: false,
    });

    commands.init_resource::<FfmpegArg>();

    app_state.set(AppState::Monitor);
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

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (dx, dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
