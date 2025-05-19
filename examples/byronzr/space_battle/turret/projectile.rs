use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::ShipPart;

use super::weapon::PhaseInfo;
use bevy::prelude::*;

// obsever 不需要在 main 中像 Event 一样注册
// commands.trigger_target(event,target) 发送局部(observe)
// commands.trigger(event) 发送全局(add_server)
#[derive(Event, Debug)]
pub struct Emit {
    pub direction: Vec2,
    pub start_position: Vec2,
}

#[derive(Component, Debug, Default)]
pub struct Projectile {
    pub damage: f32,
    pub size: f32,
    pub phase1: PhaseInfo,
    pub phase2: PhaseInfo,
}

pub fn emit_observer(
    trigger: Trigger<Emit>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO:  需要重新计算最新的向量发射

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(3.))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(255., 255., 255.)))),
        RigidBody::Dynamic,
        Collider::ball(3.),
        Friction::new(3.),
        Restitution::new(0.0),
        GravityScale(0.0),
        ColliderMassProperties::Density(1.),
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_19),
        SolverGroups::new(Group::GROUP_2, Group::GROUP_19),
        // ExternalImpulse {
        //     impulse: trigger.direction * 1000.,
        //     torque_impulse: 0.,
        // },
        Projectile::default(),
        ExternalForce {
            force: trigger.direction * 100000.,
            torque: 0.,
        },
        // 注意: 设置起始位置
        Transform::from_translation(trigger.start_position.extend(0.)),
    ));
}
