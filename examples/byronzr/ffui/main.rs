use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

mod define;
mod shortcuts;
mod ui;
mod utility;

use crate::define::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<PathDatas>()
        .add_systems(Startup, ui::setup::setup)
        .add_systems(
            Update,
            (
                shortcuts::shortcuts,
                ui::refresh_lines,
                ui::button_interaction,
                ui::process_update,
            )
                .chain(),
        )
        .run();
}
