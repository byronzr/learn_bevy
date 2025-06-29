use crate::define::*;
use bevy::prelude::*;

pub fn toggle_setting(
    mut commands: Commands,
    process_state: Res<ProcessState>,
    mut data: ResMut<PathDatas>,
) {
    // ui container
    let Some(layout_id) = process_state.layout else {
        error!("ProcessState layout is not set");
        return;
    };

    let container_id = commands
        .spawn((
            StateScoped(AppState::Monitor),
            Container,
            Node {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                //height: Val::Px(300.0), // or Val::Percent(50.0)
                position_type: PositionType::Relative,
                //margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                overflow: Overflow::scroll_y(),
                align_self: AlignSelf::Stretch,
                ..default()
            },
        ))
        .id();
    commands.entity(layout_id).add_child(container_id);
}
