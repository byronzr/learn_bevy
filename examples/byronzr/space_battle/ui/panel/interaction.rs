use bevy::prelude::*;

use crate::resources::menu::MainMenu;
use crate::resources::player::PlayerShipResource;

use super::PanelMenuButton;
use bevy_rapier2d::prelude::*;

use crate::ui::ButtonStatus;

// 注意: 世界真奇妙, Interaction 响应延时的元凶居然是 magnet.app 或  BetterTouchTool.app
// https://github.com/bevyengine/bevy/issues/10658
pub fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction, &ButtonStatus, &PanelMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: ResMut<MainMenu>,
    mut render_context: ResMut<DebugRenderContext>,
    player_res: Res<PlayerShipResource>,
) -> Result {
    for (entity, interaction, active, button) in interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            match button {
                PanelMenuButton::EngineFlame => {
                    menu.engine_flame = !menu.engine_flame;
                }
                PanelMenuButton::ShowLog => {
                    menu.log = !menu.log;
                }
                PanelMenuButton::EnemyAppear => {
                    menu.enemy_appear = !menu.enemy_appear;
                }
                PanelMenuButton::DebugRender => {
                    menu.debug_render = !menu.debug_render;
                    render_context.enabled = menu.debug_render;
                }
                PanelMenuButton::MeshMode => {
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
                PanelMenuButton::VirtualTurret => {
                    menu.virtual_turret = !menu.virtual_turret;
                }
                PanelMenuButton::DetectTest => {
                    menu.detect_test = !menu.detect_test;
                }
                PanelMenuButton::LockPlayer => {
                    menu.lock_player = !menu.lock_player;
                }
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

            // audio
            commands
                .spawn((
                    AudioPlayer::new(menu.ui_button_pressed.clone()),
                    // 万万没想到需要用这种用一次消毁一次的方法
                    PlaybackSettings::DESPAWN,
                ))
                .insert(ChildOf(entity));
        }
    }
    Ok(())
}
