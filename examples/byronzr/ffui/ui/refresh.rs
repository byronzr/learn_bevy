use bevy::prelude::*;

use crate::define::*;

// refresh lines button
pub fn refresh_lines(
    mut commands: Commands,
    mut data: ResMut<PathDatas>,
    container_query: Single<Entity, With<Container>>,
    asset_server: Res<AssetServer>,
) -> Result {
    // no changes, just return
    if !data.changed {
        return Ok(());
    }

    // clear container children
    let container_entity = *container_query;
    for entity in data.entities.iter() {
        if let Some(e) = entity {
            commands.entity(*e).despawn();
        }
    }
    data.entities.clear();

    let mut entities = vec![];
    for (index, path) in data.lines.iter().enumerate() {
        let id = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    height: Val::Px(25.0),
                    column_gap: Val::Px(5.0),
                    ..default()
                },
                children![
                    // button
                    (
                        Button,
                        IndexOfline(index),
                        Node {
                            width: Val::Px(60.),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.0)),
                        BorderColor(Color::WHITE.with_alpha(0.2)),
                        BackgroundColor(Color::srgb_u8(0, 0, 0)),
                        children![(
                            Text::new("convert"),
                            TextFont {
                                font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            //TextShadow::default(),
                        )]
                    ),
                    // path text
                    (
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Start,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(5.0)),
                        BorderColor(Color::WHITE.with_alpha(0.2)),
                        children![(
                            Text::new(path.clone()),
                            TextFont {
                                font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            //TextShadow::default(),
                        )],
                    )
                ],
            ))
            .insert(ChildOf(container_entity))
            .id();
        entities.push(Some(id));
    }
    data.entities = entities;
    data.changed = false;

    println!("showed done");
    return Ok(());
}

pub fn process_update(process_state: Res<ProcessState>) {
    let rx = &process_state.rx;
    // 处理接收的消息
    while let Ok(message) = rx.try_recv() {
        println!("Received message: {}", message);
    }
}
