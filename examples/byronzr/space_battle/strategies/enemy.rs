use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::IndexedRandom;
use rand::{Rng, rng};

use crate::components::Projectile;
use crate::components::ship::{EnemyHull, EnemyProjectPoint};
use crate::resources::enemy::EnemyGenerateTimer;
use crate::resources::menu::MainMenu;
use crate::resources::player::PlayerShipResource;
use crate::utility;

pub enum Bound {
    Left(f32),
    Top(f32),
    Right(f32),
    Bottom(f32),
}

pub fn random_enemies(
    mut commands: Commands,
    mut timer: ResMut<EnemyGenerateTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    menu: Res<MainMenu>,
) -> Result {
    if !menu.enemy_appear {
        return Ok(());
    }
    if timer.0.tick(time.delta()).just_finished() {
        // todo
        let mut rng = rng();
        // 1920 * 1080
        let axis = vec![
            Bound::Right(960.),
            Bound::Left(-960.),
            Bound::Bottom(-540.),
            Bound::Top(540.),
        ];
        // 获得一个单边轴
        let transform = match axis.choose(&mut rng) {
            Some(one) => match *one {
                Bound::Top(y) | Bound::Bottom(y) => {
                    let x = rng.random_range(-540. ..540.);
                    Transform::from_xyz(x, y, 0.)
                }
                Bound::Left(x) | Bound::Right(x) => {
                    let y = rng.random_range(-960. ..960.);
                    Transform::from_xyz(x, y, 0.)
                }
            },
            None => {
                return Ok(());
            }
        };

        let (mesh, handle, vertices) =
            utility::png::load("space_battle/tempest.png", &mut *asset_server)?;

        let mesh = Mesh2d(meshes.add(mesh));
        let material =
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.1, 0.1, 0.5))));
        let sprite = Sprite {
            image: handle.clone(),
            ..default()
        };

        let hull = commands
            .spawn((
                // MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
                EnemyHull,
                transform,
                children![EnemyProjectPoint],
            ))
            .id();
        // 添加 collider
        let Some(collider) = Collider::convex_hull(&vertices) else {
            return Err(BevyError::from("Failed to create hull collider"))?;
        };
        commands.entity(hull).insert(collider);

        if menu.mesh_mode {
            commands
                .entity(hull)
                .insert(mesh.clone())
                .insert(material.clone());
        } else {
            commands.entity(hull).insert(sprite);
        }
    }
    Ok(())
}

pub fn enemy_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, Or<(With<Projectile>, With<EnemyHull>)>>,
    res: Res<PlayerShipResource>,
) {
    let Some(player) = res.ship_entity else {
        return;
    };
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // 如果碰撞由于玩家船体则不删除
                if e1 == &player || e2 == &player {
                    continue;
                }
                // 确保非船体以外的实体被清除
                if query.contains(*e1) {
                    commands.entity(*e1).try_despawn();
                }
                if query.contains(*e2) {
                    commands.entity(*e2).try_despawn();
                }
            }
            _ => {}
        }
    }
}
