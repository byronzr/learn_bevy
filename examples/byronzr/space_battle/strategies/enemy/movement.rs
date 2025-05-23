use crate::components::{BaseVelocity, Braking};
use crate::resources::menu::MainMenu;
use crate::{components::ship::EnemyHull, utility::track::forward_to};
use bevy::prelude::*;

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<EnemyHull>>,
    time: Res<Time>,
    menu: Res<MainMenu>,
) {
    let target = Vec2::ZERO;
    let base = BaseVelocity {
        speed: 10.,
        torque: 0.5,
        braking: Braking::default(),
    };
    for (entity, mut transform) in query.iter_mut() {
        forward_to(
            &mut commands,
            &mut transform,
            300.0,
            &base,
            entity,
            target,
            time.delta_secs(),
            menu.log,
        );
    }
}
