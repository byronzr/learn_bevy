use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};
use core::time::Duration;
use rand::random;
use std::fmt;

// Begin round 1 of 10
// This game is fun!
// Alice did not score a point! Their score is: 0 (1 round cold streak)
// Bob scored a point! Their score is: 1 (1 round hot streak)
// In set 'Last' for the 1th time

// Begin round 2 of 10
// This game is fun!
// Alice did not score a point! Their score is: 0 (2 round cold streak)
// Bob did not score a point! Their score is: 1 (1 round cold streak)
// In set 'Last' for the 2th time

// Begin round 3 of 10
// This game is fun!
// Player 3 has joined the game!
// Alice did not score a point! Their score is: 0 (3 round cold streak)
// Bob scored a point! Their score is: 2 (1 round hot streak)
// Player 3 did not score a point! Their score is: 0 (1 round cold streak)
// In set 'Last' for the 3th time

// Begin round 4 of 10
// This game is fun!
// Player 4 joined the game!
// Alice scored a point! Their score is: 1 (1 round hot streak)
// Bob did not score a point! Their score is: 2 (1 round cold streak)
// Player 3 did not score a point! Their score is: 0 (2 round cold streak)
// Player 4 scored a point! Their score is: 1 (1 round hot streak)
// In set 'Last' for the 4th time

// Begin round 5 of 10
// This game is fun!
// Alice did not score a point! Their score is: 1 (1 round cold streak)
// Bob did not score a point! Their score is: 2 (2 round cold streak)
// Player 3 scored a point! Their score is: 1 (1 round hot streak)
// Player 4 did not score a point! Their score is: 1 (1 round cold streak)
// In set 'Last' for the 5th time

// Begin round 6 of 10
// This game is fun!
// Alice scored a point! Their score is: 2 (1 round hot streak)
// Bob scored a point! Their score is: 3 (1 round hot streak)
// Player 3 did not score a point! Their score is: 1 (1 round cold streak)
// Player 4 scored a point! Their score is: 2 (1 round hot streak)
// In set 'Last' for the 6th time

// Begin round 7 of 10
// This game is fun!
// Alice did not score a point! Their score is: 2 (1 round cold streak)
// Bob scored a point! Their score is: 4 (2 round hot streak)
// Player 3 scored a point! Their score is: 2 (1 round hot streak)
// Player 4 did not score a point! Their score is: 2 (1 round cold streak)
// Bob won the game!
// In set 'Last' for the 7th time

// component 就是标准的 RUST数据类型
#[derive(Component)]
struct Player {
    name: String,
}

// component 不一定只是做为静态数据，也可以作为实体（entity）存储空间
// 通过观察就知道，Score中的value会在例子中被修改。
#[derive(Component)]
struct Score {
    value: usize,
}

// 枚举同样也是可以成为 component 的,
#[derive(Component)]
enum PlayerStreak {
    Hot(usize),
    None,
    Cold(usize),
}
impl fmt::Display for PlayerStreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerStreak::Hot(n) => write!(f, "{n} round hot streak"),
            PlayerStreak::None => write!(f, "0 round streak"),
            PlayerStreak::Cold(n) => write!(f, "{n} round cold streak"),
        }
    }
}

// Resource 是一种全局的访问对象，
// 需要明确的一点是。system 作为一种多线程的表现行式，
// 那么 Resource 的成员属性也一定要满足线程同步的 trait (Sync + 'static)
#[derive(Resource, Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

#[derive(Resource)]
struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

// system 就是 实体(entities),资源（resource）,组件(component)组织起来的逻辑驱动
// system 的参数有一套比较松规则，这里展示的不需要任何参数
fn print_message_system() {
    println!("This game is fun!");
}

// 这里展示的是如何获得 Resource，在遵循RUST语言的语法前提下，
// system并不限制一次获取多个少资源，也不分先后。（但在获得前应当被初始化）
// 并且不仅是读取同样可对资源进行写入
fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!(
        "Begin round {} of {}",
        game_state.current_round, game_rules.max_rounds
    );
}

// 这里展示如何利用 component 作为条件,从 entity 中提取 component对其进行读写
// pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()>
// 从 Query 的定义就可以看到 Query的泛型中，有两个定义
// 如果需要对 component 进行可写操作需要申明
fn score_system(mut query: Query<(&Player, &mut Score, &mut PlayerStreak)>) {
    // query 永远是一个集合，大多数情况下都需要配合 for
    for (player, mut score, mut streak) in &mut query {
        let scored_a_point = random::<bool>();
        if scored_a_point {
            score.value += 1;
            *streak = match *streak {
                PlayerStreak::Hot(n) => PlayerStreak::Hot(n + 1),
                PlayerStreak::Cold(_) | PlayerStreak::None => PlayerStreak::Hot(1),
            };
            println!(
                "{} scored a point! Their score is: {} ({})",
                player.name, score.value, *streak
            );
        } else {
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
}

// 这里展示了Query与Resource的混合获取在 system 中
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

// 在这个展示了system中的消息传递的方式
fn game_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{player} won the game!");
        app_exit_writer.write(AppExit::Success);
    } else if game_state.current_round == game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_writer.write(AppExit::Success);
    }
}

// 这个函数展示的资源初始化的一种方式
// 在 system 初始化资源，是非常不确定的，
// 原因是 commands 是一个带有缓存命令，
// 除非能够能完全确保在当前system创建的资源一定优先于调用
fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    // Create our game rules resource
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    // Add some players to our world. Players start with a score of 0 ... we want our game to be
    // fair!
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

// 这里展示了一个 “高级” system,world 像是 commands 的强力版本。
// 一但使用 world,就不允许再请求其它的任何参数，因为 world 是以一种独占方式运行的。
// 当然 world 自身有很多方式获得 Entity 与 Resource
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

// 每个有 system 都可以有一个与其绑定的本地变量，
// 它的作用与 thread_local! 相似，
// 与component绑定于entity相似
fn print_at_end_round(mut counter: Local<u32>) {
    *counter += 1;
    println!("In set 'Last' for the {}th time", *counter);
    // Print an empty line between rounds
    println!();
}

// SystemSet 就是对 Schedule 的细分，
// 能够更精确的控制 system 的运行时机
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySystems {
    BeforeRound,
    Round,
    AfterRound,
}

fn main() {
    App::new()
        // 在 main 中或是在 Plugin.build() 中初始化 Resource 是一种更安全的方法
        .init_resource::<GameState>()
        // 插件就是一组调用（system）的集合，用户自定创建并没有严格的范式，
        // 但Bevy通常封包好了一些常用的 Plugin
        // 当前没有使用 DefaultPlugin 插件，所以本身程序只有命令行显示，
        // 主要设置的是Update的间隔时间
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
        // 这是最常用的三个 Schdule，为了高效运行，
        // 大多数情况下 system 是处于一种并行状态下，
        // 当需要对数据(资源，组件)进行可写访问时就会进入多线程同步贯例，
        // 所以，如无必要不要对资源和组件的读取标记为可写mut
        // Starup 仅启动时执行一次
        .add_systems(Startup, startup_system)
        // Update与Last为个“桢”间，
        .add_systems(Update, print_message_system)
        .add_systems(Last, print_at_end_round)
        // 除了使用标准的 Schedule，
        // 还可以自定义细分一个Schdule,但要注意，标准Schdule永远是最高级别。
        // 自定义细分应当被视为一种“向下”扩展
        .configure_sets(
            Update,
            // chain() 是一个通用的强制顺的方法，适用于很多场景（定义，添加）
            (
                MySystems::BeforeRound,
                MySystems::Round,
                MySystems::AfterRound,
            )
                .chain(),
        )
        // add_systems 方法的限制也非常灵活，
        // 允许增加条件(Schedule条件，State条件，system前后约定)
        // 允许嵌套
        .add_systems(
            Update,
            // 这里展了 add_systems 的灵活嵌套
            (
                (
                    (new_round_system, new_player_system).chain(),
                    exclusive_player_system,
                )
                    // 此处展示了用户自定义 SystemSet 的使用
                    .in_set(MySystems::BeforeRound),
                score_system.in_set(MySystems::Round),
                (
                    score_check_system,
                    // 这里展示了 system 前后约定的用法
                    game_over_system.after(score_check_system),
                )
                    .in_set(MySystems::AfterRound),
            ),
        )
        // 最后，一切准备就序不要忘了 run
        .run();
}
