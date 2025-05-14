use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

pub struct EnemyPlugin;

#[derive(Component)]
#[require(
    Collider::cuboid(10., 10.),
    RigidBody::Dynamic,
    GravityScale(0.),
    ColliderMassProperties::Mass(1.)
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

fn random_enemies(mut commands: Commands, mut timer: ResMut<EnemyGenerateTimer>, time: Res<Time>) {
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

        commands.spawn((EnemyHull, Transform::from_xyz(x, y, 0.)));
    }
}
