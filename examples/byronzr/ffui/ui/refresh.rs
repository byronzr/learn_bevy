use crate::define::*;
use crate::ui::{
    ui_open_button, ui_replace_button, ui_snap_button, ui_task_button, ui_task_ex_button,
};
use accesskit::{Node as Accessible, Role};
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use log::info;

// switch show and hide row
pub fn show_hide_row(
    data: ResMut<PathDatas>,
    menu: Res<ProcessMenu>,
    mut file_line_query: Query<(Entity, &mut Node), With<FileLineBar>>,
) -> Result {
    for (idx, entity) in data.entities.iter().enumerate() {
        let Some(entity) = entity else {
            continue; // skip if entity is None
        };
        if menu.hide_done && matches!(data.state.status[idx], TaskStatus::Done) {
            // despawn entity if it is done and hide_done is true
            let Ok((_, mut node)) = file_line_query.get_mut(*entity) else {
                continue;
            };
            node.display = Display::None;
        } else {
            // spawn entity if it is not done or hide_done is false
            let Ok((_, mut node)) = file_line_query.get_mut(*entity) else {
                continue;
            };
            node.display = Display::Flex;
        }
    }
    Ok(())
}

// show  import type
pub fn show_import_type(
    //process_menu: Res<ProcessMenu>,
    import_query: Query<(&MenuButton, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
) -> Result {
    for (menu, child) in import_query.iter() {
        let Some(button_type) = menu
            .button_type // button_type is a Box::new()
            .as_ref() // so,first we need to get a reference,it's Important.
            .as_any() // then we can downcast it to MenuImportButton
            .downcast_ref::<MenuImportButton>()
        else {
            continue;
        };
        // println!("MenuButton: {:?}", menu.button_type);
        // println!("ProcessMenu: {:?}", process_menu.import_type);

        let Ok(mut text) = text_query.get_mut(child[0]) else {
            continue;
        };
        text.0 = button_type.to_string();
    }
    Ok(())
}
// refresh lines when import files changed
pub fn refresh_lines(
    mut commands: Commands,
    container_query: Single<Entity, With<LinesContainer>>,
    mut data: ResMut<PathDatas>,
    font: Res<FontHandle>,
) -> Result {
    // no changes, just return
    if !data.changed {
        return Ok(());
    }

    // clear container children
    let container_entity = *container_query;
    for entity in data.entities.iter() {
        if let Some(e) = entity {
            commands.entity(*e).try_despawn();
        }
    }
    data.entities.clear();

    let mut entities = vec![];
    for (index, path) in data.state.lines.iter().enumerate() {
        let total_secs = data.state.progress.get(&index).map_or(0, |s| s.total);
        let progress = data
            .state
            .progress
            .get(&index)
            .and_then(|s| Some(s.percent))
            .unwrap_or(0.0);
        let n = 80;
        let short_path = if path.chars().count() > n {
            let start = path
                .char_indices()
                .rev()
                .nth(n - 1)
                .map(|(i, _)| i)
                .unwrap();
            &format!("... {}", &path[start..])
        } else {
            path
        };

        // create row
        let id = commands
            .spawn((
                FileLineBar,
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    column_gap: Val::Px(5.0),
                    //align_self: AlignSelf::Stretch,
                    ..default()
                },
                AccessibilityNode(Accessible::new(Role::ListItem)),
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
                children![
                    AccessibilityNode(Accessible::new(Role::ListItem)),
                    Pickable {
                        should_block_lower: false,
                        ..default()
                    },
                    // index number
                    (
                        AccessibilityNode(Accessible::new(Role::ListItem)),
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                        Node {
                            width: Val::Px(55.),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            //padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.0)),
                        //BorderColor(Color::WHITE.with_alpha(0.2)),
                        BackgroundColor(Color::WHITE.with_alpha(0.02)),
                        children![(
                            AccessibilityNode(Accessible::new(Role::ListItem)),
                            Pickable {
                                should_block_lower: false,
                                ..default()
                            },
                            Text::new(format!("{:0>3}", index + 1)),
                            TextFont {
                                font: font.0.clone(),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            //TextShadow::default(),
                        )],
                    ),
                    // total seconds
                    (
                        AccessibilityNode(Accessible::new(Role::ListItem)),
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                        Node {
                            width: Val::Px(85.),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            //padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.0)),
                        //BorderColor(Color::WHITE.with_alpha(0.2)),
                        BackgroundColor(Color::WHITE.with_alpha(0.1)),
                        children![(
                            AccessibilityNode(Accessible::new(Role::ListItem)),
                            Pickable {
                                should_block_lower: false,
                                ..default()
                            },
                            Text::new(format!("{:>}", total_secs)),
                            IndexOfline(index),
                            TextFont {
                                font: font.0.clone(),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            //TextShadow::default(),
                        )],
                    ),
                    // task button
                    ui_task_button(index, font.0.clone()),
                    // libx265
                    ui_task_ex_button(index, font.0.clone()),
                    // replace button
                    ui_replace_button(index, font.0.clone()),
                    // snapshot button A
                    ui_snap_button(index, font.0.clone(), true),
                    // snapshot button B
                    ui_snap_button(index, font.0.clone(), false),
                    // open button
                    ui_open_button(index, font.0.clone()),
                    // info layout (right)
                    (
                        AccessibilityNode(Accessible::new(Role::ListItem)),
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(1.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            row_gap: Val::Px(2.0),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.0)),
                        BorderColor(Color::WHITE.with_alpha(0.2)),
                        children![
                            // path text
                            (
                                AccessibilityNode(Accessible::new(Role::ListItem)),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Px(25.0),
                                    //border: UiRect::all(Val::Px(1.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Start,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                children![(
                                    AccessibilityNode(Accessible::new(Role::ListItem)),
                                    Pickable {
                                        should_block_lower: false,
                                        ..default()
                                    },
                                    Text::new(short_path),
                                    TextFont {
                                        font: font.0.clone(),
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                    //TextShadow::default(),
                                )],
                            ),
                            // bar
                            (
                                AccessibilityNode(Accessible::new(Role::ListItem)),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                                ProgressBar,
                                IndexOfline(index),
                                Node {
                                    width: Val::Percent(progress as f32), // initially 0%
                                    height: Val::Px(3.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb_u8(0, 250, 0)),
                            )
                        ]
                    )
                ],
            ))
            .insert(ChildOf(container_entity))
            .id();
        entities.push(Some(id));
    }
    data.entities = entities;
    data.changed = false;

    return Ok(());
}

pub fn progress_bar_update(
    mut process_state: ResMut<ProcessState>,
    mut paths_data: ResMut<PathDatas>,
    mut bar_query: Query<(&mut Node, &IndexOfline, &mut BackgroundColor), With<ProgressBar>>,
    mut text_query: Query<
        (&mut Text, &IndexOfline),
        (With<IndexOfline>, With<Text>, Without<ProgressBar>),
    >,
) {
    //let mut rx = process_state.progress_tx.subscribe();
    //paths_data.changed = true; // mark data as changed

    // 处理接收的消息
    let Ok(message) = process_state.progress_rx.try_recv() else {
        return;
    };

    let progress = &mut paths_data.state.progress;

    let Some(idx) = message.progress_index else {
        info!("Received none (index): {:?}", message);
        return;
    };

    // target progress statistics
    // Use entry API to avoid temporary value issues
    let statistics = progress.entry(idx).or_insert(ProgressStatistics {
        total: 0,
        current: 0,
        percent: 0.0,
    });

    match message.progress_type {
        ProgressType::Total => {
            statistics.total = message.progress_value;
            statistics.current = 0; // reset current when total is set
        }
        ProgressType::Current => {
            statistics.current = message.progress_value;
        }
    }

    // update percent
    statistics.percent = if statistics.total > 0 {
        (statistics.current as f64 / statistics.total as f64) * 100.0
    } else {
        0.0
    };

    for (mut node, idx, _) in bar_query.iter_mut() {
        if idx.0 == message.progress_index.unwrap() {
            // update bar width
            node.width = Val::Percent(statistics.percent as f32);
            break;
        }
    }

    // update total seconds text
    for (mut text, idx) in text_query.iter_mut() {
        if idx.0 == message.progress_index.unwrap() {
            // update text with total seconds
            text.0 = format!("{:>6}", statistics.total);
            break;
        }
    }

    // done
    if statistics.percent >= 99. {
        // change bar color to blue
        for (_, idx, mut bgcolor) in bar_query.iter_mut() {
            if idx.0 == message.progress_index.unwrap() {
                bgcolor.0 = Color::srgb_u8(0, 0, 250);
                break;
            }
        }
        paths_data.state.status[idx] = TaskStatus::Done;
    }
}
