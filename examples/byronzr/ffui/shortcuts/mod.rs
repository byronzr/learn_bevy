use arboard::Clipboard;
use bevy::{prelude::*, text::cosmic_text::ttf_parser::apple_layout::state};
use log::info;

use crate::define::*;

// observer shortcuts
pub fn shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut data: ResMut<PathDatas>,
    process_menu: Res<ProcessMenu>,
) -> Result {
    let mut clipboard = Clipboard::new()?;
    if keyboard.pressed(KeyCode::SuperLeft) && keyboard.just_pressed(KeyCode::KeyV) {
        // the contents must be a string
        let Ok(contents) = clipboard.get_text() else {
            info!("Failed to get clipboard text");
            return Ok(());
        };

        // replace all lines when type is ONCE
        let mut lines = match process_menu.import_type {
            MenuImportButton::Lock => {
                return Ok(()); // do nothing when type is LOCK
            }
            MenuImportButton::Once => {
                vec![]
            }
            MenuImportButton::Sequence => data.state.lines.clone(),
        };

        // append lines when type is SEQUENCE
        for (_index, line) in contents.lines().enumerate() {
            if !line.is_empty() && !lines.contains(&line.to_string()) {
                lines.push(line.to_string());
            }
        }
        if data.state.lines == lines {
            return Ok(());
        } else {
            data.state.lines = lines;
            data.state.status = vec![TaskStatus::Waiting; data.state.lines.len()];
            data.changed = true;
            info!("storage in PathDatas");
        }
    }
    Ok(())
}
