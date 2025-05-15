use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::enemy::EnemyHull;

#[derive(Component, Debug)]
#[require(
    RigidBody::Dynamic,
    Collider::cuboid(5., 5.),
    Friction::new(0.5),
    Restitution::new(0.5),
    ColliderMassProperties::Mass(1.0),
    GravityScale(0.0),
    CollisionGroups::new(Group::GROUP_1, Group::GROUP_19)
)]
pub struct ShipPart;

#[derive(Component, Debug)]
#[require(
    RigidBody::Dynamic,
    Collider::cuboid(10., 10.),
    Friction::new(0.5),
    Restitution::new(0.5),
    ColliderMassProperties::Mass(10.0),
    GravityScale(0.0),
    Damping{
        linear_damping: 0.5,
        angular_damping: 0.5,
    },
    CollisionGroups::new(Group::GROUP_1, Group::GROUP_19),
)]
pub struct ShipHull;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_player_ship);
        app.add_systems(Update, (drift, detect_enemy));
    }
}

#[derive(Component, Eq, PartialEq, Debug)]
enum ShipState {
    Idle,
    Moving,
}

// detect_enemy
fn detect_enemy(
    enemy_query: Populated<Entity, With<EnemyHull>>,
    player: Single<(Entity, &Transform), With<ShipHull>>,
    read_context: ReadRapierContext,
    mut gizmos: Gizmos,
) -> Result {
    let rapeir_context = read_context.single()?;
    let (entity, transform) = player.into_inner();
    let filter = QueryFilter::default().groups(CollisionGroups::new(Group::ALL, Group::GROUP_19));
    let ship_pos = transform.translation.xy();
    if let Some((entity, projection)) = rapeir_context.project_point(ship_pos, true, filter) {
        gizmos.arrow_2d(ship_pos, projection.point, Color::srgb_u8(0, 255, 255));
        println!("entity:{:?}, projection: {:?}", entity, projection);
    }

    Ok(())
}

// idle drift
fn drift(mut commands: Commands, player: Single<(Entity, &ShipState), With<ShipHull>>) {
    let (entity, state) = player.into_inner();
    if *state == ShipState::Moving {
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

// generate player
pub fn generate_player_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("space_battle/lasher_ff.png");

    // color for weapon mount
    let mount_color = materials.add(ColorMaterial::from(Color::srgb(0., 0., 0.5)));

    // 舰船头部
    let joint_bow = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 30.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    // 前左弦
    let joint_left = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(-25.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    // 前右弦
    let joint_right = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(25.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    // 后左弦
    let joint_back_left = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(-25.0, -25.0))
        .local_anchor2(Vec2::new(0.0, 0.0));
    // 后右弦
    let joint_back_right = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(25.0, -25.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    let hull = commands
        .spawn((
            ShipHull,
            // Mesh2d(meshes.add(Circle::new(10.))),
            // MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0., 0.5, 0.)))),
            Sprite {
                image: texture_handle.clone(),
                ..default()
            },
            ShipState::Idle,
        ))
        .id();

    // bow
    commands.spawn((
        ShipPart,
        ImpulseJoint::new(hull, joint_bow),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(mount_color.clone()),
        // 如果不初始化 Transform 可能会意外的产生"力"
        Transform::from_xyz(0., 30., 0.),
    ));

    // front left
    commands.spawn((
        ShipPart,
        ImpulseJoint::new(hull, joint_left),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(mount_color.clone()),
        Transform::from_xyz(-25., 0., 0.),
    ));

    // front right
    commands.spawn((
        ShipPart,
        ImpulseJoint::new(hull, joint_right),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(mount_color.clone()),
        Transform::from_xyz(25., 0., 0.),
    ));

    // back left
    commands.spawn((
        ShipPart,
        ImpulseJoint::new(hull, joint_back_left),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(mount_color.clone()),
        Transform::from_xyz(-25., -25., 0.),
    ));

    // back right
    commands.spawn((
        ShipPart,
        ImpulseJoint::new(hull, joint_back_right),
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(mount_color.clone()),
        Transform::from_xyz(25., -25., 0.),
    ));
}
