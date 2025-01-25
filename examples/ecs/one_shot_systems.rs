//! Demonstrates the use of "one-shot systems", which run once when triggered.
//!
//! These can be useful to help structure your logic in a push-based fashion,
//! reducing the overhead of running extremely rarely run systems
//! and improving schedule flexibility.
//!
//! See the [`World::run_system`](World::run_system) or
//! [`World::run_system_once`](World#method.run_system_once_with)
//! docs for more details.

use bevy::{
    ecs::system::{RunSystemOnce, SystemId},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_ui,
                // commands.spawn
                setup_with_commands,
                // world.spawn
                // 因为在 system 中有用到 world.run_system_once,所以需要在 setup_ui 之后
                setup_with_world.after(setup_ui), // since we run `system_b` once in world it needs to run after `setup_ui`
            ),
        )
        .add_systems(Update, (trigger_system, evaluate_callbacks).chain())
        .run();
}

// Callback 作为一个 system_id 的包装器
#[derive(Component)]
struct Callback(SystemId);

#[derive(Component)]
struct Triggered;

// 在这里 A/B 作来一个 Entity 的标识
#[derive(Component)]
struct A;
#[derive(Component)]
struct B;

// 将 system_a 注册到 Commands 得到 id
// 将 id 包装成 Callback component (用户定义)
// 将 Callback component 与 A component 绑定到一个 Entity 上
// spawn
fn setup_with_commands(mut commands: Commands) {
    let system_id = commands.register_system(system_a);

    // command.spawn 有缓冲区,取票排队,不会立即执行
    commands.spawn((Callback(system_id), A));
}

fn setup_with_world(world: &mut World) {
    // We can run it once manually

    // 用 world.run_system_once 执行一次
    // commands 没有 run_system_once 方法,因为 commands 是一个缓冲区
    // 所以默认屏启动时 Last B 高亮
    world.run_system_once(system_b).unwrap();

    // Or with a Callback
    let system_id = world.register_system(system_b);
    // world.spawn 会立即执行
    world.spawn((Callback(system_id), B));
}

/// Tag entities that have callbacks we want to run with the `Triggered` component.
/// 语义上是触发系统,实际上是根据输入为相应的 Entity 添加 Triggered component
/// 从而使 evaludate_callbacks 在 update schedule 中,不断地查询 Triggered component 并执行相应的 Callback
fn trigger_system(
    mut commands: Commands,
    query_a: Single<Entity, With<A>>,
    query_b: Single<Entity, With<B>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // println!("update ... update ... loop ...");

    // 当输入 A 或 B 时 使到相应的 Callback Entity
    // 然后为该 Entity 增加一个 Triggered component
    if input.just_pressed(KeyCode::KeyA) {
        let entity = *query_a;
        commands.entity(entity).insert(Triggered);
    }
    if input.just_pressed(KeyCode::KeyB) {
        let entity = *query_b;
        commands.entity(entity).insert(Triggered);
    }
}

/// Runs the systems associated with each `Callback` component if the entity also has a `Triggered` component.
///
/// This could be done in an exclusive system rather than using `Commands` if preferred.
/// 执行 拥有 Triggered component 的 Entity 的 Callback component
/// 然后,移除 Triggered component
fn evaluate_callbacks(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
    for (entity, callback) in query.iter() {
        commands.run_system(callback.0);
        commands.entity(entity).remove::<Triggered>();
    }
}

// 对 Ui 中的 Text 进行写入
fn system_a(entity_a: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_a, 3) = String::from("A");
    info!("A: One shot system registered with Commands was triggered");
}

fn system_b(entity_b: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_b, 3) = String::from("B");
    info!("B: One shot system registered with World was triggered");
}

// 创建等待输入的 UI Tips
fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 创建一个 Text Entity ,也是本例中的唯一实体
    // 符合 Single<Entity,With<Text>>
    commands
        .spawn((
            Text::default(),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
        ))
        // 明确了这个 Text Entity,由三个 TextSpan Entity 组成
        .with_children(|p| {
            p.spawn(TextSpan::new("Press A or B to trigger a one-shot system\n"));
            p.spawn(TextSpan::new("Last Triggered: "));
            p.spawn((
                TextSpan::new("-"),
                TextColor(bevy::color::palettes::css::ORANGE.into()),
            ));
        });
}
