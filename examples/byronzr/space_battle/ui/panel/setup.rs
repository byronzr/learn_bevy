use crate::ui::button;
use crate::{resources::menu::MainMenu, ui::UIResource};
use bevy::prelude::*;

use super::{PanelMenuButton, UILayoutMain};
use bevy_rapier2d::prelude::*;

pub fn ui_main_setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut menu: ResMut<MainMenu>,
    mut render_context: ResMut<DebugRenderContext>,
    ui_resource: Res<UIResource>,
) {
    // 同步 debug_render
    render_context.enabled = menu.debug_render;
    let Some(ui_layout) = ui_resource.panel else {
        return;
    };
    commands
        .spawn(button(
            &mut asset_server,
            "Show Log".into(),
            PanelMenuButton::ShowLog,
            false,
        ))
        .insert(ChildOf(ui_layout));
    commands
        .spawn(button(
            &mut asset_server,
            "Enemy Appear".into(),
            PanelMenuButton::EnemyAppear,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Debug Mode".into(),
            PanelMenuButton::DebugRender,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Mesh Mode".into(),
            PanelMenuButton::MeshMode,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Detect Test".into(),
            PanelMenuButton::DetectTest,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Virtual Turret".into(),
            PanelMenuButton::VirtualTurret,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Engine Flame".into(),
            PanelMenuButton::EngineFlame,
            false,
        ))
        .insert(ChildOf(ui_layout));

    // commands
    //     .spawn(button(
    //         &mut asset_server,
    //         format!("Weapon: {:?}", menu.weapon_type).into(),
    //         PanelMenuButton::WeaponType,
    //         false,
    //     ))
    //     .insert(ChildOf(ui_layout));

    // audio
    let sound = asset_server.load("space_battle/audio/ui_button_pressed.ogg");
    menu.ui_button_pressed = sound.clone();
}
