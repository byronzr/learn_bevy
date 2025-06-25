use crate::utility::task::task;
use crate::utility::{create_ffmpeg_command, parse_duration};
use crate::{TOKIO_RT, define::*};
use bevy::prelude::*;

// set task_button text content according to the status
pub fn update_task_button_text(
    button_query: Query<(&Children, &IndexOfline), With<TaskButton>>,
    mut text_query: Query<&mut Text>,
    data: Res<PathDatas>,
) -> Result {
    // dependent on the iterator,because there is no Interaction event
    for (children, idx) in button_query.iter() {
        // get the first child entity, which is the text entity
        let Some(childen_entity) = children.get(0) else {
            println!("No children entity found for index {}", idx.0);
            continue;
        };
        // update the text content according to the status
        if let Ok(mut text) = text_query.get_mut(*childen_entity) {
            text.0 = match data.state.status[idx.0] {
                TaskStatus::Waiting => "convert".into(),
                TaskStatus::Running => "running".to_string(),
                TaskStatus::Done => "done".to_string(),
            };
        }
    }

    Ok(())
}
// task button interaction
pub fn task_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<TaskButton>),
    >,
    mut data: ResMut<PathDatas>,
    process_state: Res<ProcessState>,
) -> Result {
    for (_entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        let Some(path) = data.state.lines.get(idx.0).cloned() else {
            return Ok(());
        };
        let status = &mut data.state.status[idx.0];
        match *interaction {
            Interaction::Hovered => {
                // set background color when the status is Waiting,otherwise do not change
                if matches!(status, TaskStatus::Waiting) {
                    *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
                }
            }
            Interaction::Pressed => {
                // skip if the status is Done
                if matches!(status, TaskStatus::Done) {
                    continue;
                }

                // start a task when status is Waiting
                if matches!(status, TaskStatus::Waiting) {
                    data.state.status[idx.0] = TaskStatus::Running;
                    *bg = BackgroundColor(Color::srgb_u8(64, 84, 64));
                    task(idx.0, &process_state, path);
                    continue;
                }
                // if the status is Running, stop the task
                if matches!(status, TaskStatus::Running) {
                    *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                    // TODO:  or interrupt task
                }
            }
            Interaction::None => match status {
                // how to revert the background color change according to the status
                TaskStatus::Waiting => {
                    // default
                    *bg = BackgroundColor(Color::srgb_u8(16, 16, 16));
                }
                TaskStatus::Running => {
                    // (green)
                    *bg = BackgroundColor(Color::srgb_u8(32, 128, 32));
                }
                TaskStatus::Done => {
                    // (blue)
                    *bg = BackgroundColor(Color::srgb_u8(32, 32, 128));
                }
            },
        }
    }
    Ok(())
}

// menu button interaction

pub fn menu_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &Name, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut process_menu: ResMut<ProcessMenu>,
) -> Result {
    for (_entity, interaction, name, mut bg) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                match name.as_str() {
                    "Lock Import" => {
                        process_menu.lock_import = !process_menu.lock_import;
                    }
                    "Hide Done" => {
                        process_menu.hide_done = !process_menu.hide_done;
                    }
                    _ => {}
                }
            }
            Interaction::None => {
                match name.as_str() {
                    "Lock Import" => {
                        if process_menu.lock_import {
                            *bg = BackgroundColor(Color::srgb_u8(64, 0, 0));
                            continue;
                        }
                    }
                    "Hide Done" => {
                        if process_menu.hide_done {
                            *bg = BackgroundColor(Color::srgb_u8(64, 0, 0));
                            continue;
                        }
                    }
                    _ => {}
                }
                *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
            }
        }
    }
    Ok(())
}
