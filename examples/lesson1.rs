//! # Hello Bevy
//!
//! 这是一个简单的 Bevy 示例，它会向你展示如何创建一个简单的 Bevy 应用程序。
//! 效果是每隔 2 秒打印一次问候消息（在终端）。

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

/// time: 在每一帧中，Bevy 会传递一个 `Time` 类型的资源，它包含了一些时间相关的信息(系统资源)。
/// timer: 我们创建了一个 `GreetTimer` 资源，它包含了一个 `Timer` 类型的值，用于计时。(定义在Plugin中，明确了间隔/重复)
/// query: 我们使用了一个 `Query` 类型的值，它用于查询所有拥有 `Name` 组件的实体。
/// 参数没有顺序要求
fn greet_people(mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>, time: Res<Time>) {
    // `time.delta()` 返回一个 `Duration` 类型的值，表示从上一帧到当前帧的时间差。
    // 这个时间差通常用于游戏的更新逻辑，例如移动物体或者更新动画。
    // 这个时间差是以秒为单位的，所以如果你的游戏运行在 60 帧每秒的帧率下，那么 `time.delta()` 大约会返回 0.01667 秒（即 1/60 秒）。

    // timer: 就是 GreetTimer 资源，它包含了一个 `Timer` 类型的值，用于计时。
    // timer.0: 通过 `timer.0` 访问 `Timer` 类型的值。(Timer，明确了2秒间隔，不断重复)
    // timeer.0.tick: 通过一个 (Duration) 时间差来更新计时器的状态,just_finished() 表示计时器是否刚刚完成。
    if timer.0.tick(time.delta()).just_finished() {
        // 遍例查询到的所有实体，并打印问候消息。
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

// 自定义插件
pub struct HelloPlugin;

// 自定义资源(时间)
#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating))) // 插入资源
            .add_systems(Startup, add_people) // 添加启动系统
            .add_systems(Update, greet_people); // 添加更新系统
    }
}

/// 程序入口点
fn main() {
    // App 是 [`bevy::app::App`]
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin)) // 默认插件 （UI系统，资源管理，2D/3D渲然）
        .run();
}

// --
// -- 如果不定义GreetTimer,是否可以正常运行？
// -- 不可以正常运行，Timer类型，无法直接插入资源，需要通过自定义资源包装一下
//
// fn greet_people(mut timer: ResMut<Timer>, query: Query<&Name, With<Person>>, time: Res<Time>) {
//     if timer.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(Timer::from_seconds(2.0, TimerMode::Repeating)) // 插入资源
//             .add_systems(Startup, add_people) // 添加启动系统
//             .add_systems(Update, greet_people); // 添加更新系统
//     }
// }
