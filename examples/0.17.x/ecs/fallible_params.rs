//! This example demonstrates how fallible parameters can prevent their systems
//! from running if their acquiry conditions aren't met.
//!
//! Fallible parameters include:
//! - [`Res<R>`], [`ResMut<R>`] - Resource has to exist.
//! - [`Single<D, F>`] - There must be exactly one matching entity.
//! - [`Option<Single<D, F>>`] - There must be zero or one matching entity.
//! - [`Populated<D, F>`] - There must be at least one matching entity.

use bevy::{ecs::error::warn, prelude::*};
use rand::Rng;

fn main() {
    println!();
    println!("Press 'A' to add enemy ships and 'R' to remove them.");
    println!("Player ship will wait for enemy ships and track one if it exists,");
    println!("but will stop tracking if there are more than one.");
    println!();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, setup)
    //     // Default system policy is to panic if parameters fail to be fetched.
    //     // We overwrite that configuration, to either warn us once or never.
    //     // This is good for catching unexpected behavior without crashing the app,
    //     // but can lead to spam.
    //     .add_systems(
    //         Update,
    //         (
    //             user_input.param_warn_once(),
    //             move_targets.never_param_warn(),
    //             move_pointer.never_param_warn(),
    //         )
    //             .chain(),
    //     )
    //     .add_systems(Update, do_nothing_fail_validation.param_warn_once())
    //     .run();

    // since 0.17.0
    App::new()
        // By default, if a parameter fail to be fetched,
        // `World::get_default_error_handler` will be used to handle the error,
        // which by default is set to panic.
        .set_error_handler(warn)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                user_input,
                move_targets,
                // since 0.17.0
                track_targets,
            )
                .chain(),
        )
        // This system will always fail validation, because we never create an entity with both `Player` and `Enemy` components.
        .add_systems(Update, do_nothing_fail_validation)
        .run();
}

/// Enemy component stores data for movement in a circle.
#[derive(Component, Default)]
struct Enemy {
    origin: Vec2,        // 旋转原点
    radius: f32,         // 旋转半径
    rotation: f32,       // 角度
    rotation_speed: f32, // 旋转速度
}

/// Player component stores data for going after enemies.
#[derive(Component, Default)]
struct Player {
    speed: f32,             // 速度
    rotation_speed: f32,    // 转旋速度
    min_follow_radius: f32, // 最小跟随半径
}

/// 创建 Player
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn 2D camera.
    commands.spawn(Camera2d);

    // Spawn player.

    let texture = asset_server.load("textures/simplespace/ship_C.png"); // 导入资产
    commands.spawn((
        Player {
            speed: 100.0,            // 速度
            rotation_speed: 2.0,     // 最小旋转速度
            min_follow_radius: 50.0, // 跟踪半径
        },
        Sprite {
            image: texture,
            color: bevy::color::palettes::tailwind::BLUE_800.into(),
            ..Default::default()
        },
        Transform::from_translation(Vec3::ZERO), // 生成在屏目中心
    ));
}

/// System that reads user input.
/// If user presses 'A' we spawn a new random enemy.
/// If user presses 'R' we remove a random enemy (if any exist).
// 通过读取 A/R 来创建/移除 enemy
fn user_input(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        let texture = asset_server.load("textures/simplespace/enemy_A.png");
        commands.spawn((
            Enemy {
                origin: Vec2::new(
                    rng.random_range(-200.0..200.0),
                    rng.random_range(-200.0..200.0),
                ), // 旋转圆点
                radius: rng.random_range(50.0..150.0), // 半径
                rotation: rng.random_range(0.0..std::f32::consts::TAU), // 角度
                rotation_speed: rng.random_range(0.5..1.5), // 速度
            },
            Sprite {
                image: texture,
                color: bevy::color::palettes::tailwind::RED_800.into(),
                ..default()
            },
            Transform::from_translation(Vec3::ZERO),
        ));
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if let Some(entity) = enemies.iter().next() {
            commands.entity(entity).despawn();
        }
    }
}

// System that moves the enemies in a circle.
// Only runs if there are enemies.
// Populated 与 Query 类似但它会保证至少拥有一个结果才会执行 system
// 对,参数结果决定是否执行 system, 你没看错
// Populated 对 Added / Changed 状态查询会有额外开销
fn move_targets(mut enemies: Populated<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    // 使 enemies 按桢更新旋转,
    for (mut transform, mut target) in &mut *enemies {
        // 计算机头方向
        target.rotation += target.rotation_speed * time.delta_secs();
        // 机头向左旋转
        transform.rotation = Quat::from_rotation_z(target.rotation);
        // 机头向右旋转
        // transform.rotation = Quat::from_rotation_z(-target.rotation);
        // 机身向左旋转
        let offset = transform.right() * target.radius;
        // 机身向右旋转
        //let offset = transform.left() * target.radius;
        transform.translation = target.origin.extend(0.0) + offset;
    }
}

/// System that moves the player.
/// The player will search for enemies if there are none.
/// If there is one, player will track it.
/// If there are too many enemies, the player will cease all action (the system will not run).
// 如果存在一个 enemy 追踪目标
// 如果太多 enemy 停止所有动作, Single 决定 system 是否运行
// never_param_warn 如果没有 会产生 panic 中止程序
// fn move_pointer(
//     // `Single` ensures the system runs ONLY when exactly one matching entity exists.
//     // 永远不会出错,因为 setup spawn 了一个 Player,但从未移除
//     // Single 确保结果只有一个 entity
//     mut player: Single<(&mut Transform, &Player)>,
//     // `Option<Single>` ensures that the system runs ONLY when zero or one matching entity exists.
//     // 有可能会出错,启动时可能不存在 enemy , 但程序依然需要合理运行,所以 Option<_>
//     enemy: Option<Single<&Transform, (With<Enemy>, Without<Player>)>>,
//     time: Res<Time>,
// ) {
//     let (player_transform, player) = &mut *player;
//     if let Some(enemy_transform) = enemy {
//         // Enemy found, rotate and move towards it.
//         // 索敌成功,开始偏移(涉及到向量计算,就不细讲了)
//         let delta = enemy_transform.translation - player_transform.translation;
//         let distance = delta.length();
//         let front = delta / distance;
//         let up = Vec3::Z;
//         let side = front.cross(up);
//         player_transform.rotation = Quat::from_mat3(&Mat3::from_cols(side, front, up));
//         let max_step = distance - player.min_follow_radius;
//         if 0.0 < max_step {
//             let velocity = (player.speed * time.delta_secs()).min(max_step);
//             player_transform.translation += front * velocity;
//         }
//     } else {
//         // No enemy found, keep searching.
//         // 索敌失败,原地自旋转
//         player_transform.rotate_axis(Dir3::Z, player.rotation_speed * time.delta_secs());
//     }
// }

/// System that moves the player, causing them to track a single enemy.
/// If there is exactly one, player will track it.
/// Otherwise, the player will search for enemies.
/// since 0.17.0 renamed from move_pointer
fn track_targets(
    // `Single` ensures the system runs ONLY when exactly one matching entity exists.
    mut player: Single<(&mut Transform, &Player)>,
    // `Option<Single>` never prevents the system from running, but will be `None` if there is not exactly one matching entity.
    enemy: Option<Single<&Transform, (With<Enemy>, Without<Player>)>>,
    time: Res<Time>,
) {
    let (player_transform, player) = &mut *player;
    if let Some(enemy_transform) = enemy {
        // Enemy found, rotate and move towards it.
        let delta = enemy_transform.translation - player_transform.translation;
        let distance = delta.length();
        let front = delta / distance;
        let up = Vec3::Z;
        let side = front.cross(up);
        player_transform.rotation = Quat::from_mat3(&Mat3::from_cols(side, front, up));
        let max_step = distance - player.min_follow_radius;
        if 0.0 < max_step {
            let velocity = (player.speed * time.delta_secs()).min(max_step);
            player_transform.translation += front * velocity;
        }
    } else {
        // 0 or multiple enemies found, keep searching.
        player_transform.rotate_axis(Dir3::Z, player.rotation_speed * time.delta_secs());
    }
}

/// This system always fails param validation, because we never
/// create an entity with both [`Player`] and [`Enemy`] components.
// 这是一个一定会触发错误的
fn do_nothing_fail_validation(_: Single<(), (With<Player>, With<Enemy>)>) {}
