use core::f32;

use crate::track;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::{
    enemy::EnemyHull,
    turret::{WeaponResource, weapon::WeaponType},
};

// 船体其它部件
#[derive(Component, Debug)]
pub struct ShipPart;

// 基础速度与扭力
#[derive(Component, Debug)]
pub struct BaseVelocity {
    speed: f32,
    torque: f32,
    braking: Braking,
}

// 基础制动系数
#[derive(Debug)]
pub struct Braking {
    distance: f32, // 制动距离
    linear: f32,   // 线性力度
    angular: f32,  // 扭力
}

// 安全距离,由武器决定
#[derive(Component, Debug)]
pub struct SafeDistance(f32);
// 船体
#[derive(Component, Debug)]
#[require(
    RigidBody::Dynamic,
    Collider::cuboid(10., 10.),
    Friction::new(0.5),
    Restitution::new(0.5),
    ColliderMassProperties::Mass(10.0),
    GravityScale(0.0),
    Damping{
        linear_damping: 0.3,
        angular_damping: 0.3,
    },
    CollisionGroups::new(Group::GROUP_1, Group::GROUP_19),
    BaseVelocity{speed:1.,torque:1.,braking:Braking{
        distance:50.,
        linear: 0.0,
        angular: 0.0,
    },},
)]
pub struct ShipHull;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_player_ship);
        app.add_systems(Update, (drift, detect_enemy));
        app.init_resource::<ShipResource>();
    }
}

#[derive(Component, Eq, PartialEq, Debug, Default)]
pub enum ShipState {
    #[default]
    Idle,
    Moving,
}

#[derive(Resource, Default)]
pub struct ShipResource {
    pub ship_entity: Option<Entity>,
    pub state: ShipState,
}

// track target
fn track_target(
    commands: &mut Commands,
    transform: &mut Transform,
    safe_distance: f32,
    base: &BaseVelocity,
    player: Entity,
    target: Vec2,
    delta_time: f32,
) {
    // 计算角度,NONE 表示无需旋转
    let rotate = track::rotaion_to(target, transform);
    if let Some((angle, clockwise)) = rotate {
        // 计算差值
        let rotation_value = clockwise * (base.torque * delta_time).min(angle);
        // 按差值旋转
        transform.rotate_z(rotation_value);
    }

    // 从飞船到目标的向量 (目标-飞船)
    let (forward, distance) = track::forward(target, transform);
    let max_step = distance - safe_distance;
    // 计算速度差值
    let velocity = (distance * base.speed * delta_time).min(max_step);
    // 速度为负数(pulse反向)
    // 有存在的必要...因为需要保持距离
    // if velocity < f32::EPSILON {
    //     return;
    // }
    // 当转向时移速会变慢
    let force = forward * velocity * if rotate.is_none() { 0.5 } else { 1.0 };
    // 施加驱动力(脉冲)
    commands.entity(player).insert(ExternalImpulse {
        impulse: force,
        torque_impulse: base.torque,
    });
}

// detect_enemy
fn detect_enemy(
    mut commands: Commands,
    _enemy_query: Populated<Entity, With<EnemyHull>>,
    player: Single<(Entity, &mut Transform, &BaseVelocity, &SafeDistance), With<ShipHull>>,
    read_context: ReadRapierContext,
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut ship: ResMut<ShipResource>,
) -> Result {
    // 出现敌人后,Populated会使 system 开始运行
    let rapeir_context = read_context.single()?;
    let (player, mut transform, base, safe) = player.into_inner();
    let filter = QueryFilter::default().groups(CollisionGroups::new(Group::ALL, Group::GROUP_19));
    let ship_pos = transform.translation.xy();
    if let Some((_enemy, projection)) = rapeir_context.project_point(ship_pos, true, filter) {
        //gizmos.arrow_2d(ship_pos, projection.point, Color::srgb_u8(0, 16, 16));
        track_target(
            &mut commands,
            &mut transform,
            safe.0,
            &base,
            player,
            projection.point,
            time.delta_secs(),
        );
        ship.state = ShipState::Moving;
    }

    Ok(())
}

// idle drift
fn drift(mut commands: Commands, player: Res<ShipResource>) {
    let Some(entity) = player.ship_entity else {
        return;
    };

    if player.state == ShipState::Moving {
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
    _asset_server: Res<AssetServer>,
    mut weapons: ResMut<WeaponResource>,
    mut ship: ResMut<ShipResource>,
) {
    //let texture_handle = asset_server.load("space_battle/lasher_ff.png");

    // color for weapon mount
    let mount_color = materials.add(ColorMaterial::from(Color::srgb(0., 0., 0.5)));

    let hull = commands
        .spawn((
            ShipHull,
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0., 0.5, 0.)))),
            // Sprite {
            //     image: texture_handle.clone(),
            //     ..default()
            // },
            SafeDistance(150.),
        ))
        .id();
    ship.ship_entity = Some(hull);
    ship.state = ShipState::Idle;

    // bow
    let bow = commands
        .spawn((
            ShipPart,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(mount_color.clone()),
            // 如果不初始化 Transform 可能会意外的产生"力"
            Transform::from_xyz(0., 30., 0.),
        ))
        .insert(ChildOf(hull))
        .id();
    weapons
        .weapon
        .push(WeaponType::Beam.init(bow, f32::EPSILON));

    // front left
    let fl = commands
        .spawn((
            ShipPart,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(mount_color.clone()),
            Transform::from_xyz(-25., 0., 0.),
        ))
        .insert(ChildOf(hull))
        .id();
    weapons.weapon.push(WeaponType::Bullet.init(fl, 45.));

    // front right
    let fr = commands
        .spawn((
            ShipPart,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(mount_color.clone()),
            Transform::from_xyz(25., 0., 0.),
        ))
        .insert(ChildOf(hull))
        .id();
    weapons.weapon.push(WeaponType::Bullet.init(fr, 45.));

    // back left
    let bl = commands
        .spawn((
            ShipPart,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(mount_color.clone()),
            Transform::from_xyz(-25., -30., 0.),
        ))
        .insert(ChildOf(hull))
        .id();
    weapons
        .weapon
        .push(WeaponType::Missile.init(bl, f32::EPSILON));

    // back right
    let br = commands
        .spawn((
            ShipPart,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(mount_color.clone()),
            Transform::from_xyz(25., -30., 0.),
        ))
        .insert(ChildOf(hull))
        .id();
    weapons
        .weapon
        .push(WeaponType::Missile.init(br, f32::EPSILON));
}
