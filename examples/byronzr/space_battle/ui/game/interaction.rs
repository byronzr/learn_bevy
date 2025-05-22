use super::GameMenuButton;
use crate::components::weapon::WeaponType;
use crate::resources::turret::TurretResource;
use crate::resources::{menu::GameMenu, player::PlayerShipResource};
use crate::ui::ButtonStatus;
use bevy::prelude::*;

pub fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (
            Entity,
            &Interaction,
            &ButtonStatus,
            &GameMenuButton,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: ResMut<GameMenu>,
    player_res: Res<PlayerShipResource>,
    mut turret_res: ResMut<TurretResource>,
    mut text_query: Query<&mut Text>,
) -> Result {
    for (entity, interaction, active, button, children) in interaction_query {
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
                    // 需要更新文本
                    let Some(text_entity) = children.get(0) else {
                        continue;
                    };
                    let mut text = text_query.get_mut(*text_entity).unwrap();
                    **text = format!("Weapon: {:?}", menu.weapon_type);
                }
                _ => {}
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
