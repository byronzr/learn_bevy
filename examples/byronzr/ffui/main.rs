use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, bounded};
use once_cell::sync::Lazy;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::runtime::Runtime;

mod define;
mod shortcuts;
mod ui;
mod utility;

use crate::define::*;

static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<PathDatas>()
        .add_systems(Startup, ui::setup::setup)
        .add_systems(
            Update,
            (
                ui::menu_interaction,
                shortcuts::shortcuts,
                ui::refresh_lines,
                ui::task_interaction,
                ui::progress_bar_update,
                ui::setup::on_window_close,
                ui::show_hide_row,
            )
                .chain(),
        )
        .run();
}
