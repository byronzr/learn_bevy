//! This example illustrates loading scenes from files.
use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::{fs::File, io::Write};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        .add_systems(
            Startup,
            (load_scene_system, infotext_system, save_scene_system),
        )
        //.add_systems(Update, replace_entity_sprite)
        .add_systems(Update, log_system)
        .add_observer(observe_component_b)
        .run();
}

// Registered components must implement the `Reflect` and `FromWorld` traits.
// The `Reflect` trait enables serialization, deserialization, and dynamic property access.
// `Reflect` enable a bunch of cool behaviors, so its worth checking out the dedicated `reflect.rs`
// example. The `FromWorld` trait determines how your component is constructed when it loads.
// For simple use cases you can just implement the `Default` trait (which automatically implements
// `FromWorld`). The simplest registered component just needs these three derives:
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

// Some components have fields that cannot (or should not) be written to scene files. These can be
// ignored with the #[reflect(skip_serializing)] attribute. This is also generally where the `FromWorld`
// trait comes into play. `FromWorld` gives you access to your App's current ECS `Resources`
// when you construct your component.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    pub path: Option<String>,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            path: None,
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}

// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

// The initial scene file will be loaded below and not change when the scene is saved
// 要读取的场景文件
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
// 要保存的场景文件
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

/// 初始化载入场景
/// 但不包括,自定义的 Sprite, 自定义的 Sprite
//// 交给了 replace_entity_sprite 处理
/// 交给了触发器
fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawning a DynamicSceneRoot creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    // Scenes can be loaded just like any other asset.
    commands.spawn(DynamicSceneRoot(asset_server.load(SCENE_FILE_PATH)));
}

// This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
// the changes appear in the console whenever you make a change.
fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}

// (触发器)只会触发一次
fn observe_component_b(
    trigger: Trigger<OnAdd, ComponentB>,
    mut query: Populated<(&mut Transform, &ComponentB)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("ComponentB added: {:?}", trigger.entity());
    for (mut transf, cb) in query.iter_mut() {
        let Some(ref path) = cb.path else {
            continue;
        };
        let entity = trigger.entity();
        transf.translation = Vec3::new(0.0, 100.0, 0.0);
        let sprite = Sprite::from_image(asset_server.load(path));
        commands.entity(entity).insert(sprite);
    }
}

// 虽然 system 被放入到 Update 阶段, 但是由于 ComponentB 只会有一次 Added,
// 虽然 system 被频繁调用,但 for 只会执行一次(所以,是有优化空间的,所以放到触发器当中)
// fn replace_entity_sprite(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Transform, &ComponentB), Added<ComponentB>>,
//     asset_server: Res<AssetServer>,
// ) {
//     for (entity, mut transf, cb) in query.iter_mut() {
//         warn!("added componentB......");
//         let Some(ref path) = cb.path else {
//             warn!("path is none");
//             return;
//         };

//         transf.translation = Vec3::new(0.0, 100.0, 0.0);
//         let sprite = Sprite::from_image(asset_server.load(path));
//         commands.entity(entity).insert(sprite);
//     }
// }

fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();
    //let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    // 读取所有注册的类型,并其放入到新的 World 中
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    // 对需要保存的组件进行处理(编排)
    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    component_b.path = Some("branding/bevy_bird_dark.png".to_string());
    scene_world.spawn((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::IDENTITY,
        Name::new("joe"),
    ));
    scene_world.spawn((ComponentA { x: 3.0, y: 4.0 }, Transform::IDENTITY));
    scene_world.insert_resource(ResourceA { score: 1 });

    // With our sample world ready to go, we can now create our scene using DynamicScene or DynamicSceneBuilder.
    // For simplicity, we will create our scene using DynamicScene:
    // 将 World 转换成 Scene
    let scene = DynamicScene::from_world(&scene_world);

    // 如果只是需要将当前的场景保存到文件,则可以直接使用下面的代码
    // 但隐藏了一个 BUG,因为某些 Entity 可能包含了未实现 Reflect 的组件,所以会导致 panic
    // 因此,在保存时,需要进行编排和清理
    // let scene = DynamicScene::from_world(&world);

    // Scenes can be serialized like this:
    // 对 scene 进行序列化(serialize)
    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in Wasm as there is no filesystem access
    // Bevy 的异步线程池
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

// This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// text example.
fn infotext_system(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Nothing to see in this window! Check the console output!"),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        Node {
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
    ));
}
