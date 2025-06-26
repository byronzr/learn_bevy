use arboard::Clipboard;
use bevy::prelude::*;
use log::info;

use crate::define::*;

// observer shortcuts
pub fn shortcuts(keyboard: Res<ButtonInput<KeyCode>>, mut data: ResMut<PathDatas>) -> Result {
    let mut clipboard = Clipboard::new()?;
    if keyboard.pressed(KeyCode::SuperLeft) && keyboard.just_pressed(KeyCode::KeyV) {
        let contents = clipboard.get_text()?;
        let mut lines = vec![];
        for (_index, line) in contents.lines().enumerate() {
            if !line.is_empty() {
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
