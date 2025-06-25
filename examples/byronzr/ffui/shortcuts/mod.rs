use arboard::Clipboard;
use bevy::prelude::*;

use crate::define::*;

// observer shortcuts
pub fn shortcuts(keyboard: Res<ButtonInput<KeyCode>>, mut data: ResMut<PathDatas>) -> Result {
    let mut clipboard = Clipboard::new()?;
    if keyboard.pressed(KeyCode::SuperLeft) && keyboard.just_pressed(KeyCode::KeyV) {
        // println!("contents: {}", clipboard.get_text()?);
        let contents = clipboard.get_text()?;
        let mut lines = vec![];
        for (_index, line) in contents.lines().enumerate() {
            if !line.is_empty() {
                lines.push(line.to_string());
            }
        }
        if data.lines == lines {
            return Ok(());
        } else {
            data.lines = lines;
            data.changed = true;
            println!("storage in PathDatas");
        }
    }
    Ok(())
}
