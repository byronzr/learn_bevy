use crate::components::ship::ShipState;
use crate::resources::player::PlayerShipResource;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

// idle drift
pub fn drift(mut commands: Commands, player: Res<PlayerShipResource>) {
    let Some(entity) = player.ship_entity else {
        return;
    };

    if player.state == ShipState::Moving {
        return;
    }
    let mut rng = rng();
    let (x, y, torque) = (
        rng.random_range(-10.0..10.0),
        rng.random_range(-10.0..10.0),
        rng.random_range(-10.0..10.0),
    );

    commands.entity(entity).insert(ExternalImpulse {
        impulse: Vec2::new(x, y),
        // 正数逆时针
        torque_impulse: torque,
    });
}
