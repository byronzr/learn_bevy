use crate::{
    components::{
        BaseVelocity,
        ship::{Hud, HudText},
    },
    resources::player::PlayerShipResource,
};
use bevy::prelude::*;

use crate::components::ship::ShipHull;

// hud 只会同步 ship 的中心点,并不需要跟随 ship 的旋转
// 这里还可以再拆分成高频同步与低频同步
pub fn sync_hud(
    mut commands: Commands,
    ship_query: Single<(&Transform, &mut BaseVelocity), (With<ShipHull>, Without<Hud>)>,
    hud_query: Single<(Entity, &mut Transform, &Children), (With<Hud>, Without<ShipHull>)>,
    hudtext_query: Query<(Entity, &HudText, &Children), With<HudText>>,
    mut text2d_query: Query<&mut Text2d>,
    mut ship_res: ResMut<PlayerShipResource>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result {
    // update value
    let (ship_transform, mut base) = ship_query.into_inner();

    // sync position
    let mut hud = hud_query.into_inner();
    hud.1.translation = ship_transform.translation;

    // new hud mesh2d
    if ship_res.pre_weapon_range != ship_res.weapon_range
        || ship_res.pre_bk_distance != ship_res.bk_distance
    {
        let mesh2d = Mesh2d(meshes.add(Annulus::new(
            ship_res.weapon_range,
            ship_res.weapon_range + ship_res.bk_distance,
        )));
        commands.entity(hud.0).insert(mesh2d);
        ship_res.pre_weapon_range = ship_res.weapon_range;
        ship_res.pre_bk_distance = ship_res.bk_distance;
    }

    // update infomation
    for (entity, hudtext, children) in hudtext_query.iter() {
        match hudtext {
            HudText::Speed => {
                if base.speed != ship_res.speed {
                    base.speed = ship_res.speed;
                    let mut text = text2d_query.get_mut(children[0])?;
                    text.0 = format!("speed: {}", base.speed);

                    // update bar
                    let sprite = hud_status_bar(base.speed, 1000.);
                    commands.entity(entity).insert(sprite);
                }
            }
            HudText::Torque => {
                if base.torque != ship_res.torque {
                    base.torque = ship_res.torque;
                    let mut text = text2d_query.get_mut(children[0])?;
                    text.0 = format!("torque: {:.2}", base.torque);

                    // update bar
                    let sprite = hud_status_bar(base.torque, 1.5);
                    commands.entity(entity).insert(sprite);
                }
            }
            HudText::BkDistance => {
                if base.braking.distance != ship_res.bk_distance {
                    base.braking.distance = ship_res.bk_distance;
                    let mut text = text2d_query.get_mut(children[0])?;
                    text.0 = format!("bk.dst: {:.2}", base.braking.distance);

                    // update bar
                    let sprite = hud_status_bar(base.braking.distance, 500.);
                    commands.entity(entity).insert(sprite);
                }
            }
            HudText::BkSpeed => {
                if base.braking.speed != ship_res.bk_speed {
                    base.braking.speed = ship_res.bk_speed;
                    let mut text = text2d_query.get_mut(children[0])?;
                    text.0 = format!("bk.speed: {:.2}", base.braking.speed);

                    // update bar
                    let sprite = hud_status_bar(base.braking.speed, 1000.);
                    commands.entity(entity).insert(sprite);
                }
            }
            HudText::BkTorque => {
                if base.braking.torque != ship_res.bk_torque {
                    base.braking.torque = ship_res.bk_torque;
                    let mut text = text2d_query.get_mut(children[0])?;
                    text.0 = format!("bk.torque: {:.2}", base.braking.torque);

                    // update bar
                    let sprite = hud_status_bar(base.braking.torque, 1.5);
                    commands.entity(entity).insert(sprite);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn hud_status_bar(min: f32, max: f32) -> Sprite {
    let values = Vec2::new(min, max).normalize();
    let sprite = Sprite {
        rect: Some(Rect::new(0., 0., 100. * values.x, 5.)),
        color: Color::srgba(1. - values.x, 1.0, 0., 0.2),
        anchor: bevy::sprite::Anchor::TopLeft,
        ..default()
    };
    sprite
}
