use bevy::prelude::*;
use std::time::Duration;

pub mod enemy;
pub mod player;
pub mod state;
pub mod turret;

use crate::components::weapon::WeaponType;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        // enemy
        app.insert_resource(enemy::EnemyGenerateTimer(Timer::new(
            Duration::from_secs(5),
            TimerMode::Repeating,
        )));

        app.init_resource::<player::PlayerShipResource>();

        // turret
        app.insert_resource(turret::TurretResource {
            weapon: vec![],
            fire_type: WeaponType::default(),
        });
    }
}
