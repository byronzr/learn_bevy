use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::{
    player::{ShipHull, ShipPart},
    switch::SwitchResource,
    turret::projectile::Projectile,
};

pub struct EnemyPlugin;

#[derive(Component)]
#[require(
    Collider::cuboid(10., 10.),
    RigidBody::Dynamic,
    GravityScale(0.),
    ColliderMassProperties::Mass(1.),
    CollisionGroups::new(Group::GROUP_19, Group::GROUP_2),
    SolverGroups::new(Group::GROUP_19, Group::GROUP_2),
    ActiveEvents::COLLISION_EVENTS
)]
pub struct EnemyHull;

#[derive(Resource)]
struct EnemyGenerateTimer(Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyGenerateTimer(Timer::new(
            Duration::from_secs(5),
            TimerMode::Repeating,
        )));
        app.add_systems(Update, (random_enemies, enemy_collision));
    }
}

fn random_enemies(
    mut commands: Commands,
    mut timer: ResMut<EnemyGenerateTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
    res: Res<SwitchResource>,
) {
    if !res.enemy_appear.1 {
        return;
    }
    if timer.0.tick(time.delta()).just_finished() {
        // todo
        let mut rng = rng();
        // 1280 * 720
        let (x, y) = (
            rng.random_range(-100. ..100.),
            rng.random_range(-100. ..100.),
        );

        // 确保在边缘
        let x = if x < 0. { -440. - x } else { 440. + x };
        let y = if y < 0. { -160. - y } else { 160. + y };

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(20., 20.))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
            EnemyHull,
            Transform::from_xyz(x, y, 0.),
        ));
    }
}

fn enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, Or<(With<Projectile>, With<EnemyHull>)>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // 确保非船体以外的实体被清除
                if query.contains(*e1) {
                    commands.entity(*e1).try_despawn();
                }
                if query.contains(*e2) {
                    commands.entity(*e2).try_despawn();
                }
            }
            _ => {}
        }
    }
}
