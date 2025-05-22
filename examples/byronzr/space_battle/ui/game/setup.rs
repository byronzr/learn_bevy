use bevy::prelude::*;

use crate::{resources::menu::GameMenu, ui::UIResource, ui::button};

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
            "Weapon: ?".into(),
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
            "Add Speed".into(),
            GameMenuButton::AddSpeed,
            false,
        ))
        .insert(ChildOf(speed_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Sub Speed".into(),
            GameMenuButton::SubSpeed,
            false,
        ))
        .insert(ChildOf(speed_id));

    // torque
    commands
        .spawn(button(
            &mut asset_server,
            "Add Torque".into(),
            GameMenuButton::AddTorque,
            false,
        ))
        .insert(ChildOf(torque_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Sub Torque".into(),
            GameMenuButton::SubTorque,
            false,
        ))
        .insert(ChildOf(torque_id));

    // BK speed
    commands
        .spawn(button(
            &mut asset_server,
            "Add BK Speed".into(),
            GameMenuButton::AddBrakingSpeed,
            false,
        ))
        .insert(ChildOf(bk_speed_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Sub BK Speed".into(),
            GameMenuButton::SubBrakingSpeed,
            false,
        ))
        .insert(ChildOf(bk_speed_id));

    // BK torque
    commands
        .spawn(button(
            &mut asset_server,
            "Add BK Torque".into(),
            GameMenuButton::AddBrakingTorque,
            false,
        ))
        .insert(ChildOf(bk_torque_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Sub BK Torque".into(),
            GameMenuButton::SubBrakingTorque,
            false,
        ))
        .insert(ChildOf(bk_torque_id));

    // BK distance
    commands
        .spawn(button(
            &mut asset_server,
            "Add BK Distance".into(),
            GameMenuButton::AddBrakingDistance,
            false,
        ))
        .insert(ChildOf(bk_distance_id));

    commands
        .spawn(button(
            &mut asset_server,
            "Sub BK Distance".into(),
            GameMenuButton::SubBrakingDistance,
            false,
        ))
        .insert(ChildOf(bk_distance_id));

    // audio
    let sound = asset_server.load("space_battle/audio/ui_type.ogg");
    menu.ui_button_pressed = sound.clone();
}
