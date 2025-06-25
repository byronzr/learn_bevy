use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, bounded};

use crate::define::*;

// initialize
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // ui container
    commands.spawn((
        Container,
        Node {
            //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            //margin: UiRect::all(Val::Px(10.0)),
            padding: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
    ));

    let (tx, rx) = bounded::<String>(100);

    commands.insert_resource(ProcessState { rx, tx });
}
