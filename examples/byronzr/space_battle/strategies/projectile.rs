use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Projectile;
use crate::components::effects::SeekFlag;
use crate::events::{Emit, SeekEnemy};
use crate::resources::player::PlayerShipResource;

pub fn emit_observer(
    trigger: Trigger<Emit>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ship: Res<PlayerShipResource>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(1.))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(3., 3., 5.)))),
        RigidBody::Dynamic,
        Collider::ball(1.),
        Friction::new(0.),
        Restitution::new(0.0),
        GravityScale(0.0),
        ColliderMassProperties::Density(1.),
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_19),
        //SolverGroups::new(Group::GROUP_2, Group::GROUP_19),
        // ExternalImpulse {
        //     impulse: trigger.direction * 1000.,
        //     torque_impulse: 0.,
        // },
        // 变小的物体速度太快会丢失
        Ccd::enabled(),
        Projectile::default(),
        ExternalForce {
            force: trigger.direction * 15000.,
            torque: 0.,
        },
        // 注意: 设置起始位置
        Transform::from_translation(trigger.start_position.extend(0.)),
    ));

    // get sound
    let Some(sound) = ship.weapon_sounds.get(&trigger.weapon_type).cloned() else {
        return;
    };

    commands.spawn((AudioPlayer::new(sound), PlaybackSettings::DESPAWN));
}

pub fn seek_observer(
    trigger: Trigger<SeekEnemy>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            SeekFlag(Timer::from_seconds(3., TimerMode::Repeating)),
            Mesh2d(meshes.add(Annulus::new(60., 61.))),
            MeshMaterial2d(materials.add(Color::srgba(0.5, 0.5, 0., 5.))),
        ))
        .insert(ChildOf(trigger.enemy_entity));
}

pub fn seek_target_clean(
    mut flags: Query<(Entity, &mut SeekFlag)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer) in flags.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
