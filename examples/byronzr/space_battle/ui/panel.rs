use bevy::prelude::*;

use crate::components::weapon::WeaponType;
use crate::resources::menu::MainMenu;
use crate::resources::player::PlayerShipResource;
use crate::resources::turret;
use crate::ui::button;

use bevy_rapier2d::prelude::*;
//use rand::{Rng, rng};

use crate::ui::ButtonStatus;

#[derive(Component, Debug)]
pub struct UILayoutMain;

#[derive(Debug, Component)]
pub enum MainMenuButton {
    ShowLog,
    EnemyAppear,
    DebugRender,
    VirtualTurret,
    DetectTest,
    WeaponType,
    MeshMode,
    None,
}

pub fn ui_main_setup(
    mut commands: Commands,
    ui_layout: Single<Entity, With<UILayoutMain>>,
    mut asset_server: ResMut<AssetServer>,
    menu: Res<MainMenu>,
    mut render_context: ResMut<DebugRenderContext>,
) {
    // 同步 debug_render
    render_context.enabled = menu.debug_render;
    let ui_layout = ui_layout.into_inner();
    commands
        .spawn(button(
            &mut asset_server,
            "Show Log".into(),
            MainMenuButton::ShowLog,
            false,
        ))
        .insert(ChildOf(ui_layout));
    commands
        .spawn(button(
            &mut asset_server,
            "Enemy Appear".into(),
            MainMenuButton::EnemyAppear,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Debug Mode".into(),
            MainMenuButton::DebugRender,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Mesh Mode".into(),
            MainMenuButton::MeshMode,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Detect Test".into(),
            MainMenuButton::DetectTest,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Virtual Turret".into(),
            MainMenuButton::VirtualTurret,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            "Engine Flame".into(),
            MainMenuButton::None,
            false,
        ))
        .insert(ChildOf(ui_layout));

    commands
        .spawn(button(
            &mut asset_server,
            format!("Weapon: {:?}", menu.weapon_type).into(),
            MainMenuButton::WeaponType,
            false,
        ))
        .insert(ChildOf(ui_layout));
}

// 注意: 世界真奇妙, Interaction 响应延时的元凶居然是 magnet.app 或  BetterTouchTool.app
// https://github.com/bevyengine/bevy/issues/10658
pub fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (
            Entity,
            &Interaction,
            &Children,
            &ButtonStatus,
            &MainMenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut menu: ResMut<MainMenu>,
    mut render_context: ResMut<DebugRenderContext>,
    player_res: Res<PlayerShipResource>,
    mut turret_res: ResMut<turret::TurretResource>,
) -> Result {
    for (entity, interaction, children, active, button) in interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            match button {
                MainMenuButton::ShowLog => {
                    menu.log = !menu.log;
                }
                MainMenuButton::EnemyAppear => {
                    menu.enemy_appear = !menu.enemy_appear;
                }
                MainMenuButton::DebugRender => {
                    menu.debug_render = !menu.debug_render;
                    render_context.enabled = menu.debug_render;
                }
                MainMenuButton::MeshMode => {
                    menu.mesh_mode = !menu.mesh_mode;
                    let Some(player_entity) = player_res.ship_entity else {
                        return Ok(());
                    };
                    let Some(sprite) = player_res.sprite.clone() else {
                        return Ok(());
                    };
                    // let Some(mesh) = player_res.mesh2d.clone() else {
                    //     return Ok(());
                    // };
                    // let Some(material) = player_res.material.clone() else {
                    //     return Ok(());
                    // };
                    if menu.mesh_mode {
                        commands.entity(player_entity).remove::<Sprite>();
                        //.insert((Mesh2d(mesh.clone()), MeshMaterial2d(material.clone())));
                    } else {
                        commands
                            .entity(player_entity)
                            // .remove::<Mesh2d>()
                            // .remove::<MeshMaterial2d<ColorMaterial>>()
                            .insert(sprite);
                    }
                }
                MainMenuButton::VirtualTurret => {
                    menu.virtual_turret = !menu.virtual_turret;
                }
                MainMenuButton::DetectTest => {
                    menu.detect_test = !menu.detect_test;
                }
                MainMenuButton::WeaponType => {
                    menu.weapon_type = match menu.weapon_type {
                        WeaponType::Beam => WeaponType::Bullet,
                        WeaponType::Bullet => WeaponType::Missile,
                        WeaponType::Missile => WeaponType::Beam,
                    };
                    //player_res
                    turret_res.fire_type = menu.weapon_type;
                    // 需要更新文本
                    let Some(text_entity) = children.get(0) else {
                        continue;
                    };
                    let mut text = text_query.get_mut(*text_entity).unwrap();
                    **text = format!("Weapon: {:?}", menu.weapon_type);
                    // no color change
                    continue;
                }
                _ => {}
            }

            // 更新按钮状态颜色
            if *active == ButtonStatus::Inactive {
                commands
                    .entity(entity)
                    .insert(ButtonStatus::Active)
                    .insert(BackgroundColor(Color::srgb_u8(0, 128, 0)));
            } else {
                commands
                    .entity(entity)
                    .insert(ButtonStatus::Inactive)
                    .insert(BackgroundColor(Color::BLACK));
            }
        }
    }
    Ok(())
}
