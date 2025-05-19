use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::switch::SwitchResource;

pub struct EnemyPlugin;

#[derive(Component)]
#[require(
    Collider::cuboid(10., 10.),
    RigidBody::Dynamic,
    GravityScale(0.),
    ColliderMassProperties::Mass(1.),
    CollisionGroups::new(Group::GROUP_19, Group::GROUP_1)
)]
pub struct EnemyHull;

#[derive(Resource)]
struct EnemyGenerateTimer(Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyGenerateTimer(Timer::new(
            Duration::from_secs(10),
            TimerMode::Repeating,
        )));
        app.add_systems(Update, random_enemies);
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
