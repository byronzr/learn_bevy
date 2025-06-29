use bevy::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

mod define;
mod shortcuts;
mod ui;
mod utility;

use crate::define::*;

static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
const FONT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/SourceHanSansCN-Normal.otf");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_resource::<PathDatas>()
        .add_systems(Startup, ui::setup::setup)
        .add_systems(OnEnter(AppState::Monitor), ui::toggle_setting)
        .add_systems(
            Update,
            (
                ui::refresh_lines,
                ui::menu_interaction,
                shortcuts::shortcuts,
                ui::task_interaction,
                ui::replace_interaction,
                ui::snapshot_interaction,
                ui::opendir_interaction,
                ui::progress_bar_update,
                ui::setup::on_window_close,
                ui::setup::update_scroll_position,
                ui::show_hide_row,
                ui::show_import_type,
                ui::update_task_button_text,
                move_or_resize_windows,
            )
                .chain(),
        )
        .run();
}

fn move_or_resize_windows(mut windows: Query<&mut Window>, input: Res<ButtonInput<MouseButton>>) {
    // Both `start_drag_move()` and `start_drag_resize()` must be called after a
    // left mouse button press as done here.
    //
    // winit 0.30.5 may panic when initiated without a left mouse button press.
    if input.just_pressed(MouseButton::Left) {
        for mut window in windows.iter_mut() {
            window.start_drag_move();
        }
    }
}
