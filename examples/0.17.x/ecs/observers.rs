//! Demonstrates how to observe life-cycle triggers as well as define custom ones.

use bevy::{
    // utils::{HashMap, HashSet},
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SpatialIndex>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_shapes, handle_click))
        // Observers are systems that run when an event is "triggered". This observer runs whenever
        // `ExplodeMines` is triggered.
        // 连环爆炸事件
        // add_observer 是(全局事件),对应的是通过 commands.trigger(ExplodeMines) 触发的事件
        .add_observer(
            // |trigger: Trigger<ExplodeMines>,
            |explode_mines: On<ExplodeMines>,
             mines: Query<&Mine>,
             index: Res<SpatialIndex>,
             mut commands: Commands| {
                // You can access the trigger data via the `Observer`
                // let event = trigger.event(); --- IGNORE --- since 0.17.0
                // Access resources
                // 读取所有临近的地雷 Entity
                for e in index.get_nearby(
                    // event.pos
                    explode_mines.pos, // since 0.17.0
                ) {
                    // Run queries
                    let mine = mines.get(e).unwrap();
                    if mine.pos.distance(explode_mines.pos) < mine.size + explode_mines.radius {
                        // And queue commands, including triggering additional events
                        // Here we trigger the `Explode` event for entity `e`
                        // 触发 `Explode` 事件, 指定该事件目的实体为 e
                        // 会被 explode_mine 接收到,它在 setup 中被注册
                        // commands.trigger_targets(Explode, e);
                        commands.trigger(Explode { entity: e });
                    }
                }
            },
        )
        // This observer runs whenever the `Mine` component is added to an entity, and places it in a simple spatial index.
        .add_observer(on_add_mine)
        // This observer runs whenever the `Mine` component is removed from an entity (including despawning it)
        // and removes it from the spatial index.
        .add_observer(on_remove_mine)
        .run();
}

// 地雷的组件与属性
#[derive(Component)]
struct Mine {
    pos: Vec2,
    size: f32,
}

impl Mine {
    fn random(rand: &mut ChaCha8Rng) -> Self {
        Mine {
            pos: Vec2::new(
                (rand.random::<f32>() - 0.5) * 1200.0,
                (rand.random::<f32>() - 0.5) * 600.0,
            ),
            size: 4.0 + rand.random::<f32>() * 16.0,
        }
    }
}

/// 这个事件的触发器,在 main 函数中,以闭包的方式注册了一个观察者
/// 爆炸传播事件
#[derive(Event)]
struct ExplodeMines {
    pos: Vec2,
    radius: f32,
}

/// 爆炸触发事件
#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 描述文本
    commands.spawn((
        Text::new(
            "Click on a \"Mine\" to trigger it.\n\
            When it explodes it will trigger all overlapping mines.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    // 方法1,通过链式调用 .observe 注册观察者函数
    commands
        .spawn(Mine::random(&mut rng))
        // Observers can watch for events targeting a specific entity.
        // This will create a new observer that runs whenever the Explode event
        // is triggered for this spawned entity.
        // .observe 是指定事件,通过 commands.trigger_targets 发送
        .observe(explode_mine);

    // We want to spawn a bunch of mines. We could just call the code above for each of them.
    // That would create a new observer instance for every Mine entity. Having duplicate observers
    // generally isn't worth worrying about as the overhead is low. But if you want to be maximally efficient,
    // you can reuse observers across entities.
    //
    // First, observers are actually just entities with the Observer component! The `observe()` functions
    // you've seen so far in this example are just shorthand for manually spawning an observer.

    // 方法2,手动创建观察者实体
    // 随后指定观察者监听的实体
    // 最后用 spawn 将 observer 实体添加到世界中
    let mut observer = Observer::new(explode_mine);

    // As we spawn entities, we can make this observer watch each of them:
    for _ in 0..1000 {
        let entity = commands.spawn(Mine::random(&mut rng)).id();
        observer.watch_entity(entity);
    }

    // By spawning the Observer component, it becomes active!
    commands.spawn(observer);
}

// 这是触发器的另一种使用方式,
// 任何实现了 Component Trait 的类型,都可以使用 Trigger<..,..> 进行监听
// 这个触发器,完成了将每个 Mine 的坐标系统一添加到 SpatialIndex 中
fn on_add_mine(
    // trigger: Trigger<OnAdd, Mine>,
    add: On<Add, Mine>, // since 0.17.0
    query: Query<&Mine>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(add.entity).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).or_default().insert(add.entity);
}

// Remove despawned mines from our index
// 同样,被个被 despawn 的 component 都会触发 OnRemove
// 将 mine 的坐标从 SpatialIndex 中清除
fn on_remove_mine(
    // trigger: Trigger<OnRemove, Mine>,
    remove: On<Remove, Mine>, // since 0.17.0
    query: Query<&Mine>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(remove.entity).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).and_modify(|set| {
        set.remove(&remove.entity);
    });
}

/// 接收到 Explode 触发事件,在 setup 中被注册
fn explode_mine(
    //trigger: Trigger<Explode>,
    explode: On<Explode>, // since 0.17.0
    query: Query<&Mine>,
    mut commands: Commands,
) {
    // If a triggered event is targeting a specific entity you can access it with `.entity()`
    // commands.trigger_targets 指定的 entity 在这里,将会被读取
    // let id = trigger.entity(); --- IGNORE --- since 0.17.0
    // let Some(mut entity) = commands.get_entity(id) else {
    let Ok(mut entity) = commands.get_entity(explode.entity) else {
        return;
    };
    info!("Boom! {:?} exploded.", explode.entity);
    // 消灭地雷
    entity.despawn();
    let mine = query.get(explode.entity).unwrap();

    // Trigger another explosion cascade.
    // 触发下一次连环爆炸
    commands.trigger(ExplodeMines {
        pos: mine.pos,
        radius: mine.size,
    });
}

// Draw a circle for each mine using `Gizmos`
// Gizmos 是一个在开发调试过程中的快速制图工具
// 注意: draw_shapes 的 shedule 是 Update 中,不断重绘桢
// 所以,不存在 "清除绘制" 的需求,
// 一但 Mine 不存在了,在下一桢就不会被绘制了
fn draw_shapes(mut gizmos: Gizmos, mines: Query<&Mine>) {
    for mine in &mines {
        gizmos.circle_2d(
            mine.pos,
            mine.size,
            Color::hsl((mine.size - 4.0) / 16.0 * 360.0, 1.0, 0.8),
        );
    }
}

// Trigger `ExplodeMines` at the position of a given click
// 在整个视窗添加一个 click 事件,当点击左键时触发并将点击位置传递给 `ExplodeMines`
// 与传统思路是为每一个gizoms添加事件,独立处理
fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Single<&Window>,
    mut commands: Commands,
) {
    // 获得相机,与全局坐标系
    let (camera, camera_transform) = *camera;
    // 如果正确获得游戏坐标系中的点击位置
    if let Some(pos) = windows
        // 获取鼠标点击位置(APP窗体)
        .cursor_position()
        // 将点击位置转换为世界(游戏中)坐标
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        // 将 Vec3 转换为 Vec2,去掉 z 坐标
        .map(|ray| ray.origin.truncate())
    {
        // 如果点击左键
        // 指定第一次爆炸范围(鼠标触发)
        // main > add_observer(||)
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // 触发 `ExplodeMines` 事件
            commands.trigger(ExplodeMines { pos, radius: 1.0 });
        }
    }
}

#[derive(Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), HashSet<Entity>>,
}

/// Cell size has to be bigger than any `TriggerMine::radius`
const CELL_SIZE: f32 = 64.0;

impl SpatialIndex {
    // Lookup all entities within adjacent cells of our spatial index
    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        let mut nearby = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(mines.iter());
                }
            }
        }
        nearby
    }
}
