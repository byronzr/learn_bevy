use crate::components::effects::EngineFlame;
use crate::components::{
    ship::{ShipHull, ShipPart, ShipState},
    weapon::WeaponType,
};
use crate::resources::{player::PlayerShipResource, turret::TurretResource};
use crate::utility;

use bevy::prelude::*;

use crate::shader::MaterialEngineFlame;
use bevy_rapier2d::prelude::*;
use core::f32;
use std::f32::consts::FRAC_PI_2;

// generate player
pub fn generate_player_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<MaterialEngineFlame>>,
    mut asset_server: ResMut<AssetServer>,
    mut turret: ResMut<TurretResource>,
    mut ship: ResMut<PlayerShipResource>,
) -> Result {
    // color for weapon mount
    let mount_color = materials.add(ColorMaterial::from(Color::srgb(0., 0., 0.5)));
    // _shape.png 文件是没有透明过渡单色文件,可为 Mesh 提供准确的轮廓,
    // 但我们使用的是纹理素材,而 rapier 优化了这个结果(只注重外轮廓)
    let (mesh, texture_handle, vertices) =
        utility::png::load("space_battle/lasher_ff.png", &mut *asset_server)?;

    // 应用勾边后的 mesh
    let mesh2d = meshes.add(mesh);
    let material = materials.add(ColorMaterial::from(Color::srgb(0., 1., 0.)));

    // 实际皮肤
    let sprite = Sprite {
        image: texture_handle.clone(),
        ..default()
    };
    ship.sprite = Some(sprite.clone());

    // 注意: ShipHull 必须要有一个 Sprite或是Mesh才能有 Transform
    // 似乎当 Mesh2d 与 Sprite 同时存在时,会异致一些不可预测的行为(错误)
    let hull = commands
        .spawn((
            ShipHull,
            sprite,
            // add Mesh2d with children
            children![(Mesh2d(mesh2d), MeshMaterial2d(material)),],
        ))
        .id();
    // 添加 collider
    let Some(collider) = Collider::convex_hull(&vertices) else {
        return Err(BevyError::from("Failed to create hull collider"))?;
    };
    commands.entity(hull).insert(collider);

    // engine flame
    // 这里使用自定义的 shader,而不是 Sprite,mesh与 vertices 被丢弃
    // let (_flame_mesh, flame_handle, _flame_vertices) =
    //     utility::png::load("space_battle/engineflame32-orig.png", &mut *asset_server)?;
    let flame_handle = asset_server.load("space_battle/engineflame32-orig.png");

    let mut transform = Transform::from_xyz(0., -45., -1.);
    transform.rotate(Quat::from_rotation_z(-FRAC_PI_2));
    transform.scale = Vec3::splat(24.);
    commands
        .spawn((
            //Mesh2d(meshes.add(flame_mesh)),
            Mesh2d(meshes.add(Rectangle::default())),
            //Mesh2d(meshes.add(Rectangle::new(48., 24.))),
            MeshMaterial2d(custom_materials.add(MaterialEngineFlame {
                my_texture: flame_handle.clone(),
                lumina: LinearRgba::WHITE,
                time: -1.,
                lumina_value: 0.0,
            })),
            transform,
            EngineFlame,
        ))
        .insert(ChildOf(hull));

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

    // default fire_type
    turret.fire_type = WeaponType::Beam;
    ship.weapon_range = turret.current_range();

    // default bk.distance
    ship.bk_distance = 10.0;
    ship.bk_speed = 1.0;
    ship.bk_torque = 0.1;

    Ok(())
}
