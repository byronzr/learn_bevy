use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;
use bevy_rapier2d::prelude::*;
//use rand::{Rng, rng};

use crate::player::ShipHull;

use crate::switch::SwitchResource;
use crate::turret::{WeaponResource, weapon::WeaponType};
use crate::ui::{ActiveButton, VirtualTurret};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_interaction);
    }
}

fn button_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction, &Children, Option<&ActiveButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut switch: ResMut<SwitchResource>,
    mut weapon: ResMut<WeaponResource>,
    mut render_context: ResMut<DebugRenderContext>,
    virtual_turret: Single<(Entity, &mut VirtualTurret, Option<&Disabled>)>,
) -> Result {
    let (detect_entity, mut virtual_turret, _) = virtual_turret.into_inner();
    for (entity, interaction, children, active) in interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            // weapon_entity
            if Some(entity) == switch.weapon_entity {
                let next = match weapon.fire_type {
                    WeaponType::Beam => WeaponType::Bullet,
                    WeaponType::Bullet => WeaponType::Missile,
                    WeaponType::Missile => WeaponType::Beam,
                };
                let mut text = text_query.get_mut(children[0])?;
                **text = format!("Weapon Type: {:?}", next);
                weapon.set_type(next);
                // 武器的处理方式是枚举,不变色
                continue;
            }

            // enemy_appear
            if Some(entity) == switch.enemy_appear.0 {
                switch.enemy_appear = (switch.enemy_appear.0, !switch.enemy_appear.1);
            }

            // detect_test
            if Some(entity) == switch.detect_test.0 {
                switch.detect_test = (switch.detect_test.0, !switch.detect_test.1);
                if switch.detect_test.1 {
                    commands.entity(detect_entity).remove::<Disabled>();
                } else {
                    commands.entity(detect_entity).insert(Disabled);
                }
            }

            // virtual_turret
            if Some(entity) == switch.virtual_turret.0 {
                switch.virtual_turret = (switch.virtual_turret.0, !switch.virtual_turret.1);
                virtual_turret.0 = switch.virtual_turret.1;
            }

            // debug_render
            if Some(entity) == switch.debug_render {
                render_context.enabled = !render_context.enabled;
                let Some(entity) = switch.background else {
                    return Ok(());
                };
                if render_context.enabled {
                    commands.entity(entity).remove::<Disabled>();
                } else {
                    commands.entity(entity).insert(Disabled);
                }
            }

            if active.is_none() {
                commands
                    .entity(entity)
                    .insert(ActiveButton)
                    .insert(BackgroundColor(Color::srgb_u8(0, 128, 0)));
            } else {
                commands
                    .entity(entity)
                    .remove::<ActiveButton>()
                    .insert(BackgroundColor(Color::BLACK));
            }
        }
    }
    Ok(())
}
