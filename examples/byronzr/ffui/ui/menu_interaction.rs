use crate::define::*;
use crate::utility::task::{open_dir, replace, snapshot, task};
use bevy::prelude::*;
use log::info;

// menu button interaction
pub fn menu_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut Name,
            &mut BackgroundColor,
            &mut MenuButton,
        ),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut process_menu: ResMut<ProcessMenu>,
    mut exit_events: EventWriter<bevy::app::AppExit>,
    preview_query: Single<Entity, With<PreviewWindow>>,
    mut data: ResMut<PathDatas>,
    mut app_state: ResMut<NextState<AppState>>,
) -> Result {
    for (_entity, interaction, name, mut bg, mut mb) in interaction_query.iter_mut() {
        let button_type = mb.button_type.as_mut();

        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb_u8(0, 84, 0));
            }
            Interaction::Pressed => {
                let checked = button_type.next();
                *bg = BackgroundColor(Color::srgb_u8(84, 84, 84));

                if let Some(bt) = button_type.as_any_mut().downcast_mut::<MenuImportButton>() {
                    process_menu.import_type = bt.clone();
                    info!("Lock import: {}", process_menu.import_type);
                }
                if button_type.as_any_mut().is::<MenuLoadButton>() {
                    info!("Load button pressed");
                    let Ok(json) = std::fs::read_to_string("files_state.json") else {
                        info!("Failed to read files_state.json");
                        continue;
                    };
                    let Ok(state) = serde_json::from_str::<FilesState>(&json) else {
                        info!("Failed to deserialize files_state.json");
                        continue;
                    };
                    data.state = state;
                    data.changed = true;
                }
                if button_type.as_any_mut().is::<MenuSaveButton>() {
                    info!("Save button pressed");

                    let Ok(json) = serde_json::to_string(&data.state) else {
                        info!("Failed to serialize PathDatas");
                        continue;
                    };
                    if let Err(e) = std::fs::write("files_state.json", json) {
                        info!("Failed to save data: {}", e);
                    } else {
                        info!("Data saved successfully");
                    }
                }
                if button_type.as_any_mut().is::<MenuClearButton>() {
                    commands.entity(*preview_query).insert(Visibility::Hidden);
                    info!("Clear preview window");
                }
                if button_type.as_any_mut().is::<MenuHideButton>() {
                    process_menu.hide_done = checked;
                    info!("Hide done tasks: {}", process_menu.hide_done);
                }
                if button_type.as_any_mut().is::<MenuToggleSetting>() {
                    process_menu.toggle_setting = checked;
                    if process_menu.toggle_setting {
                        info!("Toggle setting: ON");
                        app_state.set(AppState::Setting);
                    } else {
                        info!("Toggle setting: OFF");
                        app_state.set(AppState::Monitor);
                        data.changed = true;
                    }
                }
                if button_type.as_any_mut().is::<MenuExitButton>() {
                    exit_events.write(bevy::app::AppExit::Success);
                    continue;
                }
            }
            Interaction::None => {
                // if button_type.as_any_mut().is::<MenuImportButton>() {
                //     *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
                // }
                // if button_type.as_any_mut().is::<MenuSaveButton>() {
                //     *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
                // }
                // if button_type.as_any_mut().is::<MenuClearButton>() {
                //     *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
                // }
                // if button_type.as_any_mut().is::<MenuHideButton>() {
                //     *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
                // }
                // if button_type.as_any_mut().is::<MenuExitButton>() {
                //     *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
                // }
                *bg = BackgroundColor(Color::srgb_u8(64, 64, 64));
            }
        }
    }
    Ok(())
}
