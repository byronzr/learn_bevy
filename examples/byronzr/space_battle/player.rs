use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::{
    enemy::EnemyHull,
    weapon::{WeaponResource, WeaponType},
};

// 船体其它部件
#[derive(Component, Debug)]
pub struct ShipPart;

// 基础速度与扭力
#[derive(Component, Debug)]
struct BaseVelocity {
    speed: f32,
    torque: f32,
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
        linear_damping: 0.6,
        angular_damping: 0.7,
    },
    CollisionGroups::new(Group::GROUP_1, Group::GROUP_19),
    BaseVelocity{speed:1.,torque:1.},
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
    ship_pos: Vec2,
    target: Vec2,
    delta_time: f32,
) {
    // 最终目标方向
    let final_front = (target - ship_pos).normalize();

    // Y 轴测试
    let enemy_y = (transform.rotation * Vec3::Y).xy();
    let base_y = enemy_y.dot(final_front);
    if (base_y - 1.0).abs() > f32::EPSILON {
        // X 轴测试
        let enemy_x = (transform.rotation * Vec3::X).xy();
        let base_x = enemy_x.dot(final_front);

        // 旋转方向
        let rotation_sign = -f32::copysign(1.0, base_x);

        // 获得弧度值
        let max_angle = ops::acos(base_y.clamp(-1., 1.));

        // 计算差值
        let rotation_value = rotation_sign * (base.torque * delta_time).min(max_angle);

        // 应用旋转
        transform.rotate_z(rotation_value);
    }

    // 计算前进方向
    let delta = target - ship_pos;
    let distance = delta.length();
    let front = (transform.rotation * Vec3::Y).xy();
    let max_step = distance - safe_distance;
    let velocity = (distance * base.speed * delta_time).min(max_step);
    // transform.translation += front.extend(0.) * velocity;

    let force = front * velocity;

    // let delta = target - ship_pos;
    // let front = delta.normalize() * 100.0;
    // 变更 ship 不再是悬浮状态
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
        gizmos.arrow_2d(ship_pos, projection.point, Color::srgb_u8(0, 255, 255));
        track_target(
            &mut commands,
            &mut transform,
            safe.0,
            &base,
            player,
            ship_pos,
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
    weapons.weapon.push(WeaponType::Hamer.init(bow));

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
    weapons.weapon.push(WeaponType::Bullet.init(fl));

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
    weapons.weapon.push(WeaponType::Bullet.init(fr));

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
    weapons.weapon.push(WeaponType::Missile.init(bl));

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
    weapons.weapon.push(WeaponType::Missile.init(br));
}
