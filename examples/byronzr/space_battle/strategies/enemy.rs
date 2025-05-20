use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::components::Projectile;
use crate::components::ship::{EnemyHull, EnemyProjectPoint};
use crate::resources::enemy::EnemyGenerateTimer;
use crate::resources::player::PlayerShipResource;
use crate::resources::state::MainMenu;
use crate::utility;

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
        // 1280 * 720
        let (x, y) = (
            rng.random_range(-100. ..100.),
            rng.random_range(-100. ..100.),
        );

        // 确保在边缘
        let x = if x < 0. { -440. - x } else { 440. + x };
        let y = if y < 0. { -160. - y } else { 160. + y };

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
                Transform::from_xyz(x, y, 0.),
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
