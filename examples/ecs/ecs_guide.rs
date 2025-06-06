//! 完整的 ECS
//! --
//! https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs

//! This is a guided introduction to Bevy's "Entity Component System" (ECS)
//! All Bevy app logic is built using the ECS pattern, so definitely pay attention!
//!
//! Why ECS?
//! * Data oriented: Functionality is driven by data (面向数据,数据驱动)
//! * Clean Architecture: Loose coupling of functionality / prevents deeply nested inheritance (低契合,预防深嵌套)
//! * High Performance: Massively parallel and cache friendly (高并发,缓存优化)
//!
//! ECS Definitions:
//!
//! Component: just a normal Rust data type. generally scoped to a single piece of functionality
//!     Examples: position, velocity, health, color, name
//! 组件: 普通数据类型,用于限制一个小范围的功能(属性)
//!
//! Entity: a collection of components with a unique id
//!     Examples: Entity1 { Name("Alice"), Position(0, 0) },
//!               Entity2 { Name("Bill"), Position(10, 5) }
//! 实体: 一个组件集合,并拥有一个唯一ID
//!
//! Resource: a shared global piece of data
//!     Examples: asset storage, events, system state
//! 资源: 一个全局共享的数据
//!
//! System: runs logic on entities, components, and resources
//!     Examples: move system, damage system
//! 系统: 在实体/组件/资源上运行逻辑
//!
//! Now that you know a little bit about ECS, lets look at some Bevy code!
//! We will now make a simple "game" to illustrate what Bevy's ECS looks like in practice.

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
    utils::Duration,
};
use rand::random;
use std::fmt;

// COMPONENTS: Pieces of functionality we add to entities. These are just normal Rust data types
//

// Our game will have a number of "players". Each player has a name that identifies them
// 定义名字(玩家的)
#[derive(Component)]
struct Player {
    name: String,
}

// Each player also has a score. This component holds on to that score
// 定义分数(玩家的)
#[derive(Component)]
struct Score {
    value: usize,
}

// Enums can also be used as components.
// This component tracks how many consecutive rounds a player has/hasn't scored in.
// 回合状态(连胜,连败)
#[derive(Component)]
enum PlayerStreak {
    Hot(usize),
    None,
    Cold(usize),
}

// 状态可视输出
impl fmt::Display for PlayerStreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerStreak::Hot(n) => write!(f, "{n} round hot streak"),
            PlayerStreak::None => write!(f, "0 round streak"),
            PlayerStreak::Cold(n) => write!(f, "{n} round cold streak"),
        }
    }
}

// RESOURCES: "Global" state accessible by systems. These are also just normal Rust data types!
//

// This resource holds information about the game:
// 当前游戏状态
// (结构本身各项都实现了 Default,所以在后续代码中,可以被简单的置入)
#[derive(Resource, Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

// This resource provides rules for our "game".
// 游戏规则
// (未实现 Default ,所以后续代码中被 "系统" 自定义初始化置入)
#[derive(Resource)]

struct GameRules {
    winning_score: usize, // 赛点
    max_rounds: usize,    // 最大局数
    max_players: usize,   // 最大玩家数
}

// SYSTEMS: Logic that runs on entities, components, and resources. These generally run once each
// time the app updates.
//

// This is the simplest type of system. It just prints "This game is fun!" on each run:
// 赛前广告......
fn print_message_system() {
    println!("This game is fun!");
}

// Systems can also read and modify resources. This system starts a new "round" on each update:
// NOTE: "mut" denotes that the resource is "mutable"
// Res<GameRules> is read-only. ResMut<GameState> can modify the resource
// 赛局更新....
fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!(
        "Begin round {} of {}",
        game_state.current_round, game_rules.max_rounds
    );
}

// This system updates the score for each entity with the `Player`, `Score` and `PlayerStreak` components.
// 积分更新系统...
fn score_system(mut query: Query<(&Player, &mut Score, &mut PlayerStreak)>) {
    //遍历所有玩家
    for (player, mut score, mut streak) in &mut query {
        // 获得一个模拟得分状态
        let scored_a_point = random::<bool>();
        // 得分更新逻辑
        if scored_a_point {
            // Accessing components immutably is done via a regular reference - `player`
            // has type `&Player`.
            //
            // Accessing components mutably is performed via type `Mut<T>` - `score`
            // has type `Mut<Score>` and `streak` has type `Mut<PlayerStreak>`.
            //
            // `Mut<T>` implements `Deref<T>`, so struct fields can be updated using
            // standard field update syntax ...
            score.value += 1;
            // ... and matching against enums requires dereferencing them
            *streak = match *streak {
                PlayerStreak::Hot(n) => PlayerStreak::Hot(n + 1), // 连胜+1
                PlayerStreak::Cold(_) | PlayerStreak::None => PlayerStreak::Hot(1), // 未连胜,则连胜1
            };
            println!(
                "{} scored a point! Their score is: {} ({})",
                player.name, score.value, *streak
            );
        }
        // 未得分更新逻辑
        else {
            *streak = match *streak {
                PlayerStreak::Hot(_) | PlayerStreak::None => PlayerStreak::Cold(1),
                PlayerStreak::Cold(n) => PlayerStreak::Cold(n + 1),
            };

            println!(
                "{} did not score a point! Their score is: {} ({})",
                player.name, score.value, *streak
            );
        }
    }

    // this game isn't very fun is it :)
}

// This system runs on all entities with the `Player` and `Score` components, but it also
// accesses the `GameRules` resource to determine if a player has won.
// 玩家胜出状态更新
fn score_check_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Player, &Score)>,
) {
    for (player, score) in &query {
        if score.value == game_rules.winning_score {
            game_state.winning_player = Some(player.name.clone());
        }
    }
}

// This system ends the game if we meet the right conditions. This fires an AppExit event, which
// tells our App to quit. Check out the "event.rs" example if you want to learn more about using
// events.
// 终局状态更新
fn game_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{player} won the game!");
        app_exit_events.send(AppExit::Success);
    } else if game_state.current_round == game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_events.send(AppExit::Success);
    }
}

// This is a "startup" system that runs exactly once when the app starts up. Startup systems are
// generally used to create the initial "state" of our game. The only thing that distinguishes a
// "startup" system from a "normal" system is how it is registered:
//      Startup: app.add_systems(Startup, startup_system)
//      Normal:  app.add_systems(Update, normal_system)
// 用在启动时的系统,只会运行一次,通常是对(各类)"状态"的一次初始化
fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    // Create our game rules resource
    // 定义游戏规则
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    // Add some players to our world. Players start with a score of 0 ... we want our game to be
    // fair!
    //初始化玩家实体的状态
    commands.spawn_batch(vec![
        (
            Player {
                name: "Alice".to_string(),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ),
        (
            Player {
                name: "Bob".to_string(),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ),
    ]);

    // set the total players to "2"
    game_state.total_players = 2;
}

// This system uses a command buffer to (potentially) add a new player to our game on each
// iteration. Normal systems cannot safely access the World instance directly because they run in
// parallel. Our World contains all of our components, so mutating arbitrary parts of it in parallel
// is not thread safe. Command buffers give us the ability to queue up changes to our World without
// directly accessing it
// commands.spawn 会以排队的方式避免系统直接修改(World)中的实例.
fn new_player_system(
    mut commands: Commands,
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
) {
    // Randomly add a new player
    let add_new_player = random::<bool>();
    if add_new_player && game_state.total_players < game_rules.max_players {
        game_state.total_players += 1;
        commands.spawn((
            Player {
                name: format!("Player {}", game_state.total_players),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ));

        println!("Player {} joined the game!", game_state.total_players);
    }
}

// If you really need full, immediate read/write access to the world or resources, you can use an
// "exclusive system".
// WARNING: These will block all parallel execution of other systems until they finish, so they
// should generally be avoided if you want to maximize parallelism.
// world.spawn 会以独占的方式避免同步竟争修改(world)中的实例
fn exclusive_player_system(world: &mut World) {
    // this does the same thing as "new_player_system"
    let total_players = world.resource_mut::<GameState>().total_players;
    let should_add_player = {
        let game_rules = world.resource::<GameRules>();
        let add_new_player = random::<bool>();
        add_new_player && total_players < game_rules.max_players
    };
    // Randomly add a new player
    if should_add_player {
        println!("Player {} has joined the game!", total_players + 1);
        world.spawn((
            Player {
                name: format!("Player {}", total_players + 1),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ));

        let mut game_state = world.resource_mut::<GameState>();
        game_state.total_players += 1;
    }
}

// Sometimes systems need to be stateful. Bevy's ECS provides the `Local` system parameter
// for this case. A `Local<T>` refers to a value of type `T` that is owned by the system.
// This value is automatically initialized using `T`'s `FromWorld`* implementation upon the system's initialization.
// In this system's `Local` (`counter`), `T` is `u32`.
// Therefore, on the first turn, `counter` has a value of 0.
//
// *: `FromWorld` is a trait which creates a value using the contents of the `World`.
// For any type which is `Default`, like `u32` in this example, `FromWorld` creates the default value.
fn print_at_end_round(mut counter: Local<u32>) {
    *counter += 1;
    println!("In set 'Last' for the {}th time", *counter);
    // Print an empty line between rounds
    println!();
}

/// A group of related system sets, used for controlling the order of systems. Systems can be
/// added to any number of sets.
// 更大的作用 将一个 {Update...} 拆分成更多语义清晰的更新节点
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySet {
    BeforeRound,
    Round,
    AfterRound,
}

// Our Bevy app's entry point
fn main() {
    // Bevy apps are created using the builder pattern. We use the builder to add systems,a
    // resources, and plugins to our app
    App::new()
        // Resources that implement the Default or FromWorld trait can be added like this:
        .init_resource::<GameState>()
        // Plugins are just a grouped set of app builder calls (just like we're doing here).
        // We could easily turn our game into a plugin, but you can check out the plugin example for
        // that :) The plugin below runs our app's "system schedule" once every 5 seconds.
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
        // `Startup` systems run exactly once BEFORE all other systems. These are generally used for
        // app initialization code (ex: adding entities and resources)
        .add_systems(Startup, startup_system)
        // `Update` systems run once every update. These are generally used for "real-time app logic"
        .add_systems(Update, print_message_system)
        // SYSTEM EXECUTION ORDER
        //
        // Each system belongs to a `Schedule`, which controls the execution strategy and broad order
        // of the systems within each tick. The `Startup` schedule holds
        // startup systems, which are run a single time before `Update` runs. `Update` runs once per app update,
        // which is generally one "frame" or one "tick".
        //
        // By default, all systems in a `Schedule` run in parallel, except when they require mutable access to a
        // piece of data. This is efficient, but sometimes order matters.
        // For example, we want our "game over" system to execute after all other systems to ensure
        // we don't accidentally run the game for an extra round.
        //
        // You can force an explicit ordering between systems using the `.before` or `.after` methods.
        // Systems will not be scheduled until all of the systems that they have an "ordering dependency" on have
        // completed.
        // There are other schedules, such as `Last` which runs at the very end of each run.
        .add_systems(Last, print_at_end_round)
        // We can also create new system sets, and order them relative to other system sets.
        // Here is what our games execution order will look like:
        // "before_round": new_player_system, new_round_system
        // "round": print_message_system, score_system
        // "after_round": score_check_system, game_over_system
        .configure_sets(
            Update,
            // chain() will ensure sets run in the order they are listed
            (MySet::BeforeRound, MySet::Round, MySet::AfterRound).chain(),
        )
        // The add_systems function is powerful. You can define complex system configurations with ease!
        .add_systems(
            Update,
            (
                // These `BeforeRound` systems will run before `Round` systems, thanks to the chained set configuration
                (
                    // You can also chain systems! new_round_system will run first, followed by new_player_system
                    (new_round_system, new_player_system).chain(),
                    exclusive_player_system,
                )
                    // All of the systems in the tuple above will be added to this set
                    .in_set(MySet::BeforeRound),
                // This `Round` system will run after the `BeforeRound` systems thanks to the chained set configuration
                score_system.in_set(MySet::Round),
                // These `AfterRound` systems will run after the `Round` systems thanks to the chained set configuration
                (
                    score_check_system,
                    // In addition to chain(), you can also use `before(system)` and `after(system)`. This also works
                    // with sets!
                    game_over_system.after(score_check_system),
                )
                    .in_set(MySet::AfterRound),
            ),
        )
        // This call to run() starts the app we just built!
        .run();
}
