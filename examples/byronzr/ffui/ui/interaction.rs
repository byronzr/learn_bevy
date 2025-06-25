use crate::utility::task::task;
use crate::utility::{create_ffmpeg_command, parse_duration};
use crate::{TOKIO_RT, define::*};
use bevy::prelude::*;
use std::io::BufRead;
use tokio::io::{AsyncBufReadExt, BufReader};

// task button interaction
pub fn task_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<TaskButton>),
    >,
    data: Res<PathDatas>,
    process_state: Res<ProcessState>,
) -> Result {
    for (_entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                let Some(path) = data.state.lines.get(idx.0).cloned() else {
                    return Ok(());
                };
                println!("convert path: {}", path);
                // start a task
                task(idx.0, &process_state, path);
                // TODO:  or interrupt task
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb_u8(0, 0, 0));
            }
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
