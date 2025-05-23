use bevy::prelude::*;

use crate::{
    components::weapon::WeaponType,
    resources::menu::GameMenu,
    ui::{UIResource, button},
};

use super::GameMenuButton;

pub fn ui_game_setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut menu: ResMut<GameMenu>,
    ui_resource: Res<UIResource>,
) {
    let Some(layout) = ui_resource.game else {
        return;
    };
    commands
        .spawn(button(
            &mut asset_server,
            format!("Weapon: {:?}", WeaponType::Beam).into(),
            GameMenuButton::WeaponType,
            false,
        ))
        .insert(ChildOf(layout));

    let speed_id = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        })
        .insert(ChildOf(layout))
        .id();

    let torque_id = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        })
        .insert(ChildOf(layout))
        .id();

    let bk_speed_id = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        })
        .insert(ChildOf(layout))
        .id();

    let bk_torque_id = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        })
        .insert(ChildOf(layout))
        .id();

    let bk_distance_id = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        })
        .insert(ChildOf(layout))
        .id();

    // speed
    commands
        .spawn(button(
            &mut asset_server,
            "Speed +100".into(),
            GameMenuButton::AddSpeed,
            false,
        ))
        .insert(ChildOf(speed_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Speed -10".into(),
            GameMenuButton::SubSpeed,
            false,
        ))
        .insert(ChildOf(speed_id));

    // torque
    commands
        .spawn(button(
            &mut asset_server,
            "Torque +0.05".into(),
            GameMenuButton::AddTorque,
            false,
        ))
        .insert(ChildOf(torque_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Torque -0.05".into(),
            GameMenuButton::SubTorque,
            false,
        ))
        .insert(ChildOf(torque_id));

    // BK speed
    commands
        .spawn(button(
            &mut asset_server,
            "BK Speed +100".into(),
            GameMenuButton::AddBrakingSpeed,
            false,
        ))
        .insert(ChildOf(bk_speed_id));

    commands
        .spawn(button(
            &mut asset_server,
            "BK Speed -10".into(),
            GameMenuButton::SubBrakingSpeed,
            false,
        ))
        .insert(ChildOf(bk_speed_id));

    // BK torque
    commands
        .spawn(button(
            &mut asset_server,
            "BK Torque +0.05".into(),
            GameMenuButton::AddBrakingTorque,
            false,
        ))
        .insert(ChildOf(bk_torque_id));

    commands
        .spawn(button(
            &mut asset_server,
            "BK Torque -0.05".into(),
            GameMenuButton::SubBrakingTorque,
            false,
        ))
        .insert(ChildOf(bk_torque_id));

    // BK distance
    commands
        .spawn(button(
            &mut asset_server,
            "BK Distance +10".into(),
            GameMenuButton::AddBrakingDistance,
            false,
        ))
        .insert(ChildOf(bk_distance_id));

    commands
        .spawn(button(
            &mut asset_server,
            "BK Distance -5".into(),
            GameMenuButton::SubBrakingDistance,
            false,
        ))
        .insert(ChildOf(bk_distance_id));

    // audio
    let sound = asset_server.load("space_battle/audio/ui_type.ogg");
    menu.ui_button_pressed = sound.clone();
}
