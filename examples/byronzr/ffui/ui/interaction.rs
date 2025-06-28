use crate::define::*;
use crate::utility::task::{open_dir, replace, snapshot, task};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use log::info;

// set task_button text content according to the status
pub fn update_task_button_text(
    button_query: Query<(&Children, &IndexOfline, &TaskButtonType), With<TaskButton>>,
    mut text_query: Query<&mut Text>,
    data: Res<PathDatas>,
) -> Result {
    // dependent on the iterator,because there is no Interaction event
    for (children, idx, btty) in button_query.iter() {
        // get the first child entity, which is the text entity
        let Some(childen_entity) = children.get(0) else {
            info!("No children entity found for index {}", idx.0);
            continue;
        };
        // update the text content according to the status
        if let Ok(mut text) = text_query.get_mut(*childen_entity) {
            text.0 = match data.state.status[idx.0] {
                TaskStatus::Waiting => {
                    if btty.0 {
                        "sf".into()
                    } else {
                        "hw".into()
                    }
                }
                TaskStatus::Running => "running".to_string(),
                TaskStatus::Done => "done".to_string(),
                TaskStatus::Replaced => "replaced".to_string(),
            };
        }
    }

    Ok(())
}
// task button interaction
pub fn task_interaction(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &IndexOfline,
            &mut BackgroundColor,
            &TaskButtonType,
        ),
        (Changed<Interaction>, With<TaskButton>),
    >,
    mut data: ResMut<PathDatas>,
    process_state: Res<ProcessState>,
) -> Result {
    for (_entity, interaction, idx, mut bg, btty) in interaction_query.iter_mut() {
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
                match status {
                    // start a task when status is Waiting
                    TaskStatus::Waiting => {
                        data.state.status[idx.0] = TaskStatus::Running;
                        *bg = BackgroundColor(Color::srgb_u8(64, 84, 64));
                        task(idx.0, &process_state, path, btty.0);
                        continue;
                    }
                    // if the status is Running, stop the task
                    TaskStatus::Running => {
                        *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                        // TODO:  or interrupt task
                    }
                    // skip if the status is Done and Replaced
                    TaskStatus::Done | TaskStatus::Replaced => {
                        continue;
                    }
                }

                if matches!(status, TaskStatus::Running) {}
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
                TaskStatus::Done | TaskStatus::Replaced => {
                    // (blue)
                    *bg = BackgroundColor(Color::srgb_u8(32, 32, 128));
                }
            },
        }
    }
    Ok(())
}

pub fn replace_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<ReplaceButton>),
    >,
    mut data: ResMut<PathDatas>,
) -> Result {
    for (_entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        let Some(path) = data.state.lines.get(idx.0).cloned() else {
            return Ok(());
        };
        let status = &mut data.state.status[idx.0];
        if matches!(status, TaskStatus::Replaced) {
            continue;
        }
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                // replace the source file only when the status is Done
                if matches!(status, TaskStatus::Done) {
                    replace(idx.0, path, &mut data);
                }
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb_u8(16, 16, 16));
            }
        }
    }
    Ok(())
}

pub fn snapshot_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<SnapshotButton>),
    >,
    mut data: ResMut<PathDatas>,
    preview_query: Single<Entity, With<PreviewWindow>>,
    mut images: ResMut<Assets<Image>>,
) -> Result {
    for (_entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        let Some(path) = data.state.lines.get(idx.0).cloned() else {
            return Ok(());
        };

        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                // replace the source file only when the status is Done
                // if matches!(status, TaskStatus::Done) {
                //     snapshot(path);
                // }

                let buf = snapshot(path);
                let img = image::load_from_memory(&buf).unwrap();
                let preview_entity = *preview_query;
                let bevy_img = bevy::image::Image::from_dynamic(
                    img,
                    true,
                    bevy::render::render_asset::RenderAssetUsages::default(),
                );
                let handle = images.add(bevy_img);

                commands.entity(preview_entity).insert((
                    Visibility::Visible,
                    ImageNode {
                        image: handle,
                        ..default()
                    },
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb_u8(16, 16, 16));
            }
        }
    }
    Ok(())
}

pub fn opendir_interaction(
    mut interaction_query: Query<
        (Entity, &Interaction, &IndexOfline, &mut BackgroundColor),
        (Changed<Interaction>, With<OpenButton>),
    >,
    data: Res<PathDatas>,
) -> Result {
    for (_entity, interaction, idx, mut bg) in interaction_query.iter_mut() {
        let Some(path) = data.state.lines.get(idx.0).cloned() else {
            return Ok(());
        };
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                // replace the source file only when the status is Done
                open_dir(path);
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb_u8(16, 16, 16));
            }
        }
    }
    Ok(())
}

// menu button interaction
pub fn menu_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &Name, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut process_menu: ResMut<ProcessMenu>,
    mut exit_events: EventWriter<bevy::app::AppExit>,
    preview_query: Single<Entity, With<PreviewWindow>>,
) -> Result {
    for (_entity, interaction, name, mut bg) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));
                match name.as_str() {
                    "Lock" => {
                        process_menu.lock_import = !process_menu.lock_import;
                    }
                    "Hide" => {
                        process_menu.hide_done = !process_menu.hide_done;
                    }
                    "Clear" => {
                        //process_menu.hide_done = !process_menu.hide_done;
                        commands.entity(*preview_query).insert(Visibility::Hidden);
                    }
                    "Exit" => {
                        exit_events.write(bevy::app::AppExit::Success);
                        continue;
                    }
                    _ => {}
                }
            }
            Interaction::None => {
                match name.as_str() {
                    "Lock" => {
                        if process_menu.lock_import {
                            *bg = BackgroundColor(Color::srgb_u8(64, 0, 0));
                            continue;
                        }
                    }
                    "Hide" => {
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
