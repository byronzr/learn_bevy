use super::GameMenuButton;
use crate::components::weapon::WeaponType;
use crate::resources::turret::TurretResource;
use crate::resources::{menu::GameMenu, player::PlayerShipResource};
use crate::ui::ButtonStatus;
use bevy::prelude::*;

pub fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction, &GameMenuButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: ResMut<GameMenu>,
    mut player_res: ResMut<PlayerShipResource>,
    mut turret_res: ResMut<TurretResource>,
    mut text_query: Query<&mut Text>,
) -> Result {
    for (entity, interaction, button, children) in interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            match button {
                GameMenuButton::WeaponType => {
                    menu.weapon_type = match menu.weapon_type {
                        WeaponType::Beam => WeaponType::Bullet,
                        WeaponType::Bullet => WeaponType::Missile,
                        WeaponType::Missile => WeaponType::Beam,
                    };
                    //player_res
                    turret_res.fire_type = menu.weapon_type;
                    player_res.weapon_range = turret_res.current_range();
                    // 需要更新文本
                    let Some(text_entity) = children.get(0) else {
                        continue;
                    };
                    let mut text = text_query.get_mut(*text_entity).unwrap();
                    **text = format!("Weapon: {:?}", menu.weapon_type);
                }
                GameMenuButton::AddSpeed => player_res.speed += 100.0,
                GameMenuButton::SubSpeed => {
                    player_res.speed -= if player_res.speed > 0. { 10.0 } else { 0. }
                }
                GameMenuButton::AddTorque => player_res.torque += 0.05,
                GameMenuButton::SubTorque => {
                    player_res.torque -= if player_res.torque > 0. { 0.05 } else { 0. }
                }
                GameMenuButton::AddBrakingSpeed => player_res.bk_speed += 100.0,
                GameMenuButton::SubBrakingSpeed => {
                    player_res.bk_speed -= if player_res.bk_speed > 0. { 10.0 } else { 0. }
                }
                GameMenuButton::AddBrakingTorque => player_res.bk_torque += 0.05,
                GameMenuButton::SubBrakingTorque => {
                    player_res.bk_torque -= if player_res.bk_torque > 0. { 0.05 } else { 0. }
                }
                GameMenuButton::AddBrakingDistance => player_res.bk_distance += 10.0,
                GameMenuButton::SubBrakingDistance => {
                    player_res.bk_distance -= if player_res.bk_distance > 0. { 5.0 } else { 0. }
                }
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

        if matches!(interaction, Interaction::Hovered) {
            // 更新按钮状态颜色
            commands
                .entity(entity)
                .insert(ButtonStatus::Active)
                .insert(BackgroundColor(Color::srgb_u8(0, 128, 128)));
        } else {
            commands
                .entity(entity)
                .insert(ButtonStatus::Inactive)
                .insert(BackgroundColor(Color::BLACK));
        }
    }
    Ok(())
}
