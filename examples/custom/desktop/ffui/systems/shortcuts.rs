use crate::utility::analyze_ffprobe_command;
use crate::{TOKIO_RT, define::*};
use arboard::Clipboard;
use bevy::prelude::*;
use tokio::sync::mpsc;

// observer shortcuts
pub fn shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut data: ResMut<PathDatas>,
    process_menu: Res<ProcessMenu>,
    mut process_state: ResMut<ProcessState>,
    ffmpeg_args: Res<FfmpegArg>,
) -> Result {
    let mut clipboard = Clipboard::new()?;

    if keyboard.pressed(KeyCode::SuperLeft) && keyboard.just_pressed(KeyCode::KeyV) {
        // the contents must be a string
        let Ok(contents) = clipboard.get_text() else {
            //info!("Failed to get clipboard text");
            process_state
                .toast_message
                .push("Failed to get clipboard text".to_string());
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
            let args = ffmpeg_args.analyze.clone();
            // start analyze duration
            analyze_duration(
                data.state.lines.clone(),
                process_state.progress_tx.clone(),
                args,
                process_state.toast_tx.clone(),
            );
            data.changed = true;
            //info!("storage in PathDatas");
            process_state
                .toast_message
                .push("Failed to get clipboard text".to_string());
        }
    }
    Ok(())
}

fn analyze_duration(
    lines: Vec<String>,
    tx: mpsc::Sender<ProgressInfo>,
    args: Vec<ArgKeyValue>,
    toast_tx: mpsc::Sender<String>,
) {
    std::thread::spawn(move || {
        for (index, line) in lines.iter().enumerate() {
            TOKIO_RT.block_on(async {
                let mut cmd = analyze_ffprobe_command(line.to_string(), &args);
                let total_secs = match cmd.output().await {
                    Ok(output) => {
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            if let Some(duration) = stdout.lines().next() {
                                // parse f64 from str
                                duration.parse::<f64>().unwrap_or_else(|_| {
                                    //info!("Failed to parse duration from output: {}", duration);

                                    0.0 // default to 0.0 if parsing fails
                                })
                            } else {
                                0.0 // default to 0 if no duration found
                            }
                        } else {
                            // info!(
                            //     "ffprobe command failed: {}",
                            //     String::from_utf8_lossy(&output.stderr)
                            // );

                            0.0 // default to 0 on failure
                        }
                    }
                    Err(e) => {
                        //info!("ffprobe command error: {}", e);
                        let msg = format!("ffprobe command error: {}", e);
                        let _ = toast_tx.send(msg).await;
                        0.0 // default to 0 on error
                    }
                };
                let msg = format!("analyze duration: {} secs", total_secs);
                let _ = toast_tx.send(msg).await;
                let _ = tx.send(ProgressInfo::total(total_secs as u64, index)).await;
            });
        }
    });
}
