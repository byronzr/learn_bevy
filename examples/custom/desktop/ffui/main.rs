use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

mod define;
mod systems;
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
        .add_plugins((InputDispatchPlugin, TabNavigationPlugin))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_resource::<PathDatas>()
        .add_systems(Startup, ui::setup::setup)
        .add_systems(OnEnter(AppState::Monitor), ui::enter_monitor)
        .add_systems(OnEnter(AppState::Setting), ui::enter_setting)
        .add_systems(Update, ui::focus_system.run_if(in_state(AppState::Setting)))
        .add_systems(
            Update,
            (
                systems::task_interaction,
                systems::replace_interaction,
                systems::snapshot_interaction,
                systems::opendir_interaction,
                systems::menu_interaction,
                systems::update_task_button_text,
                systems::toast_animate,
                systems::toast_consumer,
                systems::toast_receiver,
                systems::move_or_resize_windows,
                systems::shortcuts,
            ),
        )
        .add_systems(
            Update,
            (
                ui::refresh_lines,
                ui::progress_bar_update,
                ui::setup::on_window_close,
                ui::setup::update_scroll_position,
                ui::show_hide_row,
                ui::show_import_type,
            )
                .chain(),
        )
        .run();
}
