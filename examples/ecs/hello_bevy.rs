//! # Hello Bevy
//!
//! 这是一个简单的 Bevy 示例，它会向你展示如何创建一个简单的 Bevy 应用程序。
//! 效果是每隔 2 秒打印一次问候消息（在终端）。
//! Entities, Components, Systems
//! bevy 有一个核心准则,<模块化>,未来不知道,但是现在对于开发人员(程序员)来说是十分友好的.所以 bevy 提倡高度插件(Plugin)化

use bevy::prelude::*;

// --------------------------------------------------
// 与一些传统设计观念不一样的是，Bevy 并不是基于面向对象的设计模式。
// Component 是 Bevy 中的一个重要概念，它是一种数据类型，用于描述实体的属性。
// 而这仿佛是满天的星星,你可以先为星星明确一个名字.
// 当你觉得有必要时,你可以把一组星星编组成为一个星座.
#[derive(Component)]
struct Person;

// --------------------------------------------------
// Name 描述的是一个名字
#[derive(Component)]
struct Name(String);

// --------------------------------------------------
// 一个对实体生成的封装,这种封装的 "类型" 为 "system"
// 通常,不同个数的 system 都会封装到 Plugin(插件) 里
fn add_people(mut commands: Commands) {
    // 下面生产了三个实体,它们都拥有 `Person` 组件，但是每个实体都有不同的 `Name` 组件。
    // commands.spawn(()); 用于创建一个实体，它会返回一个 `EntityCommands` 类型的值。
    // () 在 bevy 是被 macro_rules! tuple_impl 实现了 Bundle
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// --------------------------------------------------
// system (无修改 Query)
// **********
// time: 在每一帧中，Bevy 会传递一个 `Time` 类型的资源，它包含了一些时间相关的信息(系统资源)。
// timer: 我们创建了一个 `GreetTimer` 资源，它包含了一个 `Timer` 类型的值，用于计时。(定义在Plugin中，明确了间隔/重复)
// query: 我们使用了一个 `Query` 类型的值，它用于查询所有拥有 `Name` 组件的实体。
// 参数没有顺序要求
fn greet_people(mut timer: ResMut<GreetTimer>, time: Res<Time>, query: Query<&Name, With<Person>>) {
    // `time.delta()` 返回一个 `Duration` 类型的值，表示从上一帧到当前帧的时间差。
    // 这个时间差通常用于游戏的更新逻辑，例如移动物体或者更新动画。
    // 这个时间差是以秒为单位的，所以如果你的游戏运行在 60 帧每秒的帧率下，那么 `time.delta()` 大约会返回 0.01667 秒（即 1/60 秒）。

    // timer: 就是 GreetTimer 资源，它包含了一个 `Timer` 类型的值，用于计时。
    // timeer.0.tick: 通过一个 (Duration) 时间推进 timer,
    // 如果是一个非循环的 timer 将会被钳停止
    // 如果是一个循环的 timer 将会标记后继续推进,不会暂停
    // just_finished() 表示计时器是否刚刚完成(tick)。
    // timer 会更新自已的进度与状态,所以需要 mut 与 ResMut
    if timer.0.tick(time.delta()).just_finished() {
        // 遍例查询到的所有实体，并打印问候消息。
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

// --------------------------------------------------
// System (可修改 Query)
// **********
// --------------------------------------------------

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        // Elaina Proctor 不会是显示的,因为 greet_people 中的计时器,跳过了
        // 当 just_finished() == true 时,已经被没有计时器的 update_people 更改了
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

// 自定义插件
pub struct HelloPlugin;

// --------------------------------------------------
// 自定义资源(时间)
// Timer 仅管 GreetTimer 在这里显得多此一举,
// 实际上是通过 derive(Resource) 将一个计时器转换成为一个名为 GreetTimer 的资源
#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating))) // 插入资源
            .add_systems(Startup, add_people) // 添加启动系统
            .add_systems(Update, (greet_people, update_people).chain()); // 添加更新系统
    }
}

// 程序入口点
fn main() {
    // App 是 [`bevy::app::App`]
    App::new()
        // 可用(, , ,) 一次性加入,也可以单独加入
        //.add_plugins((DefaultPlugins, HelloPlugin))
        .add_plugins(DefaultPlugins) // 默认插件 （UI系统，资源管理，2D/3D渲然）
        .add_plugins(HelloPlugin)
        .run();
}
