use crate::components::ship::EnemyProjectPoint;
use crate::components::{BaseVelocity, Braking};
use crate::resources::menu::MainMenu;
use crate::resources::player::PlayerShipResource;
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
        braking: Braking {
            speed: 1.0,
            torque: 0.5,
            distance: 10.0,
        },
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

pub fn enemy_locked(
    mut commands: Commands,
    mut query: Query<Entity, With<EnemyProjectPoint>>,

    ship: Res<PlayerShipResource>,
    mut mesheds: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter_mut() {
        if let Some(target_entity) = ship.target_enmey {
            if entity == target_entity {
                commands
                    .spawn((
                        Mesh2d(mesheds.add(Annulus::new(60., 61.))),
                        MeshMaterial2d(materials.add(Color::srgba(0.5, 0., 0., 5.))),
                    ))
                    .insert(ChildOf(entity));
                break;
            }
        }
    }
}
