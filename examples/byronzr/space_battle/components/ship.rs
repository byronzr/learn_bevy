use super::{BaseVelocity, Braking};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// 敌舰主体(也是标识)
#[derive(Component)]
#[require(
    //Collider::cuboid(10., 10.),
    RigidBody::Dynamic,
    GravityScale(0.),
    ColliderMassProperties::Mass(10.),
    CollisionGroups::new(Group::GROUP_19, Group::GROUP_2|Group::GROUP_1),
    ActiveEvents::COLLISION_EVENTS,
    BaseVelocity{
        speed:1.,
        torque:1.,
        braking:Braking{
            distance:50.,
            speed: 1.0,
            torque: 1.0,
        },
    },
)]
pub struct EnemyHull;

// 用于进行投射的敌舰核心点,因为当舰体旋转时测试轮廓会索敌抖动
#[derive(Component, Debug)]
#[require(
    Collider::cuboid(0.1, 0.1), // 只用于投射,越小越好
    CollisionGroups::new(Group::GROUP_18, Group::ALL),
    // 注意: 投射点并不存在 Transform 所以我们要自已手动加入
    Transform::default(),
)]
pub struct EnemyProjectPoint;

// 玩家船体其它部件
#[derive(Component, Debug)]
pub struct ShipPart;

// 玩家船体(也是标识)
#[derive(Component, Debug)]
#[require(
    RigidBody::Dynamic,
    Friction::new(0.5),
    Restitution::new(0.5),
    ColliderMassProperties::Mass(10.0),
    GravityScale(0.0),
    Damping{
        linear_damping: 0.1,
        angular_damping: 0.1,
    },
    CollisionGroups::new(Group::GROUP_1, Group::GROUP_19),
    BaseVelocity{
        speed:1.,
        torque:1.,
        braking:Braking{
            distance:50.,
            speed: 1.0,
            torque: 1.0,
        },
    },
)]
pub struct ShipHull;

// 玩家船体当前状态(服务于漂移)
#[derive(Component, Eq, PartialEq, Debug, Default)]
pub enum ShipState {
    #[default]
    Idle,
    Moving,
}

#[derive(Debug, Component)]
pub struct Hud;

#[derive(Debug, Component, Clone, Copy)]
pub enum HudText {
    Flux,
    Capacity,
    Speed,
    Torque,
    BkSpeed,
    BkTorque,
    BkDistance,
    Cooldown,
    Hitpoint,
}
