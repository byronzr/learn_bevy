use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
#[require(
    RigidBody::Dynamic,
    Collider::cuboid(5., 5.),
    Friction::new(0.5),
    Restitution::new(0.5),
    ColliderMassProperties::Mass(1.0),
    GravityScale(0.0)
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
)]
pub struct ShipHull;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_player);
    }
}

pub fn generate_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // color for hull
    let hull_color = materials.add(ColorMaterial::from(Color::srgb(0., 0.5, 0.)));

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
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(hull_color),
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
