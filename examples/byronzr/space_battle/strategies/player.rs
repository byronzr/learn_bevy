use crate::components::{
    BaseVelocity, SafeDistance,
    ship::{EnemyHull, EnemyProjectPoint, ShipHull, ShipPart, ShipState},
    weapon::WeaponType,
};
use crate::resources::{player::PlayerShipResource, turret::TurretResource};
use crate::utility::{self, track};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;
use rand::{Rng, rng};

use super::{enemy, projectile};

// track target
pub fn track_target(
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
    if velocity < f32::EPSILON {
        return;
    }
    // 当转向时移速会变慢
    let force = forward * velocity * if rotate.is_none() { 0.5 } else { 1.0 };
    // 施加驱动力(脉冲)
    commands.entity(player).insert(ExternalImpulse {
        impulse: force,
        torque_impulse: base.torque,
    });
}

// player_detection
pub fn player_detection(
    mut commands: Commands,
    // 注意: 测试投射点是 EnemyProjectPoint,而不是 EnemyHull
    enemy_query: Populated<
        (Entity, &GlobalTransform),
        (With<EnemyProjectPoint>, Without<ShipHull>),
    >,
    // 注意: ShipHull 必须要有一个 Sprite或是Mesh才能有 Transform
    player: Single<
        (Entity, &mut Transform, &BaseVelocity, &SafeDistance),
        (With<ShipHull>, Without<EnemyHull>),
    >,
    read_context: ReadRapierContext,
    time: Res<Time>,
    mut ship: ResMut<PlayerShipResource>,
) -> Result {
    // 出现敌人后,Populated会使 system 开始运行
    let rapeir_context = read_context.single()?;
    let (player, mut transform, base, safe) = player.into_inner();
    // 注意: 投射查询的是 EnemyProjectPoint,而不是 EnemyHull
    let filter = QueryFilter::default().groups(CollisionGroups::new(Group::ALL, Group::GROUP_18));
    let ship_pos = transform.translation.xy();
    // 敌人消失后,才会进行新的测试,防止策略摇摆
    let point = if let Some(enemy) = ship.target_enmey {
        if let Ok((_, transform)) = enemy_query.get(enemy) {
            transform.translation().xy()
        } else {
            // 能够运行到这里,说明还有敌人存在,只是未进行投射,所以,我们将 target 设为 None
            // 下次就进行投射了
            ship.target_enmey = None;
            Vec2::ZERO
        }
    } else {
        if let Some((enemy, projection)) = rapeir_context.project_point(ship_pos, true, filter) {
            ship.state = ShipState::Moving;
            ship.target_enmey = Some(enemy);
            projection.point
        } else {
            // 完全没有敌人了
            Vec2::ZERO
        }
    };

    // 进行跟踪
    track_target(
        &mut commands,
        &mut transform,
        safe.0,
        &base,
        player,
        point,
        time.delta_secs(),
    );

    Ok(())
}

// idle drift
pub fn drift(mut commands: Commands, player: Res<PlayerShipResource>) {
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
    mut asset_server: ResMut<AssetServer>,
    mut turret: ResMut<TurretResource>,
    mut ship: ResMut<PlayerShipResource>,
) -> Result {
    // color for weapon mount
    let mount_color = materials.add(ColorMaterial::from(Color::srgb(0., 0., 0.5)));
    // _shape.png 文件是没有透明过渡单色文件,可为 Mesh 提供准确的轮廓,
    // 但我们使用的是纹理素材,而 rapier 优化了这个结果(只注重外轮廓)
    // let (mesh, texture_handle, vertices) =
    //     resources::png::load_png("space_battle/lasher_ff_shape.png", &mut *asset_server)?;

    let (mesh, texture_handle, vertices) =
        utility::png::load("space_battle/lasher_ff.png", &mut *asset_server)?;

    let mesh2d = meshes.add(mesh);
    let material = materials.add(ColorMaterial::from(Color::srgb(0., 0.5, 0.)));

    let sprite = Sprite {
        image: texture_handle.clone(),
        ..default()
    };
    ship.sprite = Some(sprite.clone());
    ship.mesh2d = Some(mesh2d.clone());
    ship.material = Some(material.clone());

    // 注意: ShipHull 必须要有一个 Sprite或是Mesh才能有 Transform
    // 似乎当 Mesh2d 与 Sprite 同时存在时,会异致一些不可预测的行为(错误)
    let hull = commands.spawn((ShipHull, sprite)).id();
    // 添加 collider
    let Some(collider) = Collider::convex_hull(&vertices) else {
        return Err(BevyError::from("Failed to create hull collider"))?;
    };
    commands.entity(hull).insert(collider);

    // 记录飞船与状态
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
    turret.weapon.push(WeaponType::Beam.init(bow, f32::EPSILON));

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
    turret.weapon.push(WeaponType::Bullet.init(fl, 45.));

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
    turret.weapon.push(WeaponType::Bullet.init(fr, 45.));

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
    turret
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
    turret
        .weapon
        .push(WeaponType::Missile.init(br, f32::EPSILON));
    Ok(())
}
