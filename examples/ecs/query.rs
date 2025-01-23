//! 模拟一个玩家遭遇战的场景
//! 敌人一共有 4 个,共有 4 种类型
//!
//! Boss
//! 红色怪
//! 绿色怪
//! 隐形怪
//!
//! 3种攻击方式
//!
//! (B)omb, 爆炸 除 BOSS 以外都扣血
//! (L)ash, 鞭打 除 Player 和隐身怪 以外,都扣血
//! (G)lare,所有敌方人员强制 BodyColor::White (隐身怪不再隐身)
//!
//! --------------------------------------------------
//! 1. 涉及到的内容 System Pipe 的补充
//! 2. Query 的用法 QueryData 与 QueryFilter
//! 3. Query 的延伸方法 Single 与 Populated,外加 Option<Single>
use bevy::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum QueryError {
    #[error("none")]
    None,
}

// (B)omb, 爆炸 除 BOSS 以外都扣血
// (L)ash, 鞭打 除 Player 和隐身怪 以外,都扣血
// (G)lare,所有敌方人员强制 BodyColor::White (隐身怪不再隐身)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(NotifyPlayerTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .insert_resource(NotifyEnemiesTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            choice
                .pipe(bomb_all)
                .pipe(lash_enemy)
                .pipe(glare_all)
                .map(in_choice_map),
        )
        .add_systems(Update, refresh_all)
        .add_systems(Update, (notify_player, notify_enemies).chain())
        .run();
}

// 初始化
fn setup(mut commands: Commands) {
    // 一个玩家
    commands.spawn((Player, Health(100), Armor));
    //commands.spawn((Player, Health(100), Armor));

    // 一个红色的精英敌人
    commands.spawn((Enemy, BodyColor::Red, Armor, Health(30)));
    // 一个绿色的普通敌人
    commands.spawn((Enemy, BodyColor::Green, Health(15)));
    // 一个隐身的敌人
    commands.spawn((Enemy, Health(1)));
    // 一个 boss
    commands.spawn((Boss, BodyColor::Red, Health(100), Armor));
}

// 等待用户选择
fn choice(input: Res<ButtonInput<KeyCode>>) -> Result<KeyCode, QueryError> {
    let Some(code) = input.get_just_pressed().next() else {
        return Err(QueryError::None);
    };

    Ok(*code)
}

// 当不存在玩家后,Single无法执行,并且会 panic
fn notify_player(
    time: Res<Time>,
    mut timer: ResMut<NotifyPlayerTimer>,
    player: Option<Single<EntityRef, With<Player>>>, // 获取到 entity 的组件
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let Some(player) = player else {
        println!("no player alive.");
        return;
    };

    println!("----------------------");
    println!("(B)omb -60, (L)ash -10, (G)lare");
    let Some(health) = player.get::<Health>() else {
        return;
    };
    let Some(armor) = player.get::<Armor>() else {
        return;
    };
    println!(
        "player ({}): health: {:?} armor: {:?} ",
        player.id(),
        health,
        armor
    );
}

// 如果存在一个或多个敌人,则打印所有敌人的信息
// 如果不存在敌人,则不执行
fn notify_enemies(
    time: Res<Time>,
    mut timer: ResMut<NotifyEnemiesTimer>,
    enemies: Populated<
        (
            Entity,
            &Health,
            Option<&Armor>,
            Option<&BodyColor>,
            Option<&Enemy>,
            Option<&Boss>,
        ),
        Without<Player>,
    >,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    println!("----------------------");
    for (entity, health, armor, bc, enemy, boss) in enemies.iter() {
        print!("entity: ({})", entity);
        print!(" ({})", **health);
        print!(" / ({:?})", armor);
        if let Some(color) = bc {
            print!(" / {:?}", color);
        }
        if enemy.is_some() {
            print!(" / enemy");
        }
        if boss.is_some() {
            print!(" / boss");
        }
        println!();
    }
    println!();
}

// 所有敌方人员强制 BodyColor::White
fn glare_all(
    In(key): In<Result<KeyCode, QueryError>>, // 串联的参数,需要有序前置
    mut query: Query<(Entity, Option<&mut BodyColor>), Without<Player>>,
    mut commands: Commands,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };
    if code == KeyCode::KeyG {
        for (entity, bg) in &mut query {
            if let Some(mut bc) = bg {
                *bc = BodyColor::White
            } else {
                // entity insert component(BodyColor::White)
                commands.entity(entity).insert(BodyColor::White);
            }
        }
    }
    key
}

// 除 Player与隐身怪 以外,都扣血
fn lash_enemy(
    In(key): In<Result<KeyCode, QueryError>>, // 串联的参数,需要有序前置
    mut query: Query<&mut Health, (Without<Player>, With<BodyColor>)>,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };

    if code == KeyCode::KeyL {
        let lash_damage = 10;
        for mut health in &mut query {
            **health -= lash_damage;
        }
    }
    key
}

// 假设需求是丢一个炸弹,场景生物都要扣血.(除了BOSS)
// 但作为玩家,受到的伤害与敌人不同,所以需要分别处理
// 所以我们用了 两个 query 来分别处理
// 并且,我们需要串联一个遍历操作,来看看执行结果
fn bomb_all(
    In(key): In<Result<KeyCode, QueryError>>, // 串联的参数,需要有序前置
    // 从逻辑上似乎是没有错的,rust-analyzer 也没有报错,但是编译器会报错
    // error[B0001]: Query<&mut query::Health, bevy_ecs::query::filter::With<query::Enemy>> in system query::bomb_all accesses component(s) query::Health in a way that conflicts with a previous system parameter. Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`. See: https://bevyengine.org/learn/errors/b0001
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
    //player_query: Query<&mut Health, With<Player>>,
    //enemy_query: Query<&mut Health, With<Enemy>>,

    // 可能有些多余,但正确的作法是加入 without
    // mut player_query: Query<&mut Health, (With<Player>,)>,
    // mut enemy_query: Query<&mut Health, (With<Enemy>,)>,
    mut player_query: Query<&mut Health, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Health, (With<Enemy>, Without<Player>)>,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };

    if code == KeyCode::KeyB {
        // bomb 的伤害
        let bomb_damage = 60;

        // 玩家只受到一半伤害
        for mut health in player_query.iter_mut() {
            // health.0 -= bomb_damage / 2;
            // 因为有 Deref 与  DerefMut 的存在,所以可以直接使用 ** 来获取/修改到值
            **health -= bomb_damage / 2;
        }

        // 敌人受到全额伤害
        for mut health in enemy_query.iter_mut() {
            **health -= bomb_damage;
        }
    }
    key
}

// 清理所有 health 小于 0 的 entity
fn refresh_all(query: Query<(Entity, &Health), With<Health>>, mut commands: Commands) {
    for (entity, health) in &query {
        if **health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

// map 传参相对简单,但不是一个 system,只是作为一个后续的结果处理器
fn in_choice_map(key: Result<KeyCode, QueryError>) {
    if let Ok(code) = key {
        println!("map show: ({:?})\n\n", code)
    }
}

#[derive(Resource)]
struct NotifyPlayerTimer(Timer);

#[derive(Resource)]
struct NotifyEnemiesTimer(Timer);

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Enemy;

#[derive(Component, Debug, Deref, DerefMut)]
struct Health(i64);

#[derive(Component, Debug)]
struct Armor;

#[derive(Component, Debug)]
struct Boss;

#[derive(Component, Debug)]
enum BodyColor {
    Red,
    Green,
    White,
}
