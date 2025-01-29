//! This example illustrates the different ways you can employ component lifecycle hooks.
//!
//! Whenever possible, prefer using Bevy's change detection or Events for reacting to component changes.
//! Events generally offer better performance and more flexible integration into Bevy's systems.
//! Hooks are useful to enforce correctness but have limitations (only one hook per component,
//! less ergonomic than events).
//!
//! Here are some cases where components hooks might be necessary:
//!
//! - Maintaining indexes: If you need to keep custom data structures (like a spatial index) in
//!     sync with the addition/removal of components.
//!
//! - Enforcing structural rules: When you have systems that depend on specific relationships
//!     between components (like hierarchies or parent-child links) and need to maintain correctness.

// 通常 Events 性能更好,更加灵活.
// 但 hook 在维护索引与组织结构时,更据优势

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use std::collections::HashMap;

#[derive(Debug)]
/// Hooks can also be registered during component initialization by
/// using [`Component`] derive macro:
/// ```no_run
/// #[derive(Component)]
/// #[component(on_add = ..., on_insert = ..., on_replace = ..., on_remove = ...)]
/// ```
struct MyComponent(KeyCode);

impl Component for MyComponent {
    // Table ,迭代更快
    // SparseSet, 增删更快
    const STORAGE_TYPE: StorageType = StorageType::Table;

    /// Hooks can also be registered during component initialization by
    /// implementing `register_component_hooks`
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        // Register hooks...
        println!("// Register hooks...// Register hooks...");
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct MyComponentIndex(HashMap<KeyCode, Entity>);

#[derive(Event)]
struct MyEvent;

fn main() {
    std::env::set_var("NO_COLOR", "1");
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册 hook closure
        .add_systems(Startup, setup)
        // 模拟 component 的动态添加与删除,以触发 hook
        .add_systems(Update, trigger_hooks)
        // 在 on_add 中触发 hook 需要更新的索引
        .init_resource::<MyComponentIndex>()
        // 模拟在 hook 中发送事件(但并没有实际意义)
        .add_event::<MyEvent>()
        .run();
}

fn setup(world: &mut World) {
    // In order to register component hooks the component must:
    // - not be currently in use by any entities in the world
    // - not already have a hook of that kind registered
    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
    world
        .register_component_hooks::<MyComponent>()
        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_replace` and `on_remove`
        // A hook has 3 arguments:
        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
        // - the entity that triggered the hook
        // - the component id of the triggering component, this is mostly used for dynamic components
        //
        // `on_add` will trigger when a component is inserted onto an entity without it
        // 当发现有新的 Component 添加到某个实体中时触发
        // 从实体中获得 component 再从 component 中获得 keycode
        // 将 keycode 与 entity 添加到资源索引(HashMap)
        .on_add(|mut world, entity, component_id| {
            // You can access component data from within the hook
            let value = world.get::<MyComponent>(entity).unwrap().0;
            println!("Component: {component_id:?} added to: {entity:?} with value {value:?}");
            // Or access resources
            world
                .resource_mut::<MyComponentIndex>()
                .insert(value, entity);
            // Or send events
            world.send_event(MyEvent);
        })
        // `on_insert` will trigger when a component is inserted onto an entity,
        // regardless of whether or not it already had it and after `on_add` if it ran
        // 当发现一个新的 component 添加到实体时触发
        // 不管该实体是否已拥有 component 都会在 on_add 后执行
        .on_insert(|world, _, _| {
            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
        })
        // `on_replace` will trigger when a component is inserted onto an entity that already had it,
        // and runs before the value is replaced.
        // Also triggers when a component is removed from an entity, and runs before `on_remove`
        // 当替换 component 时会触发
        // 同样也会在 on_remove 前触发
        // 获得 keycode 后,按其从资源索引中移除
        .on_replace(|mut world, entity, _| {
            let value = world.get::<MyComponent>(entity).unwrap().0;
            world.resource_mut::<MyComponentIndex>().remove(&value);
            println!("on_replace: {:?}", world.resource::<MyComponentIndex>());
        })
        // `on_remove` will trigger when a component is removed from an entity,
        // since it runs before the component is removed you can still access the component data
        // 从 world 中移除 entity
        .on_remove(|mut world, entity, component_id| {
            let value = world.get::<MyComponent>(entity).unwrap().0;
            println!("Component: {component_id:?} removed from: {entity:?} with value {value:?}");
            // You can also issue commands through `.commands()`
            // 从 world 中移除 entity
            world.commands().entity(entity).despawn();
        });
}

/// 一个等待用户输入的来 spawn entity 的 system
fn trigger_hooks(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    index: Res<MyComponentIndex>,
) {
    //从资源 index 中检出 key 与 entity
    for (key, entity) in index.iter() {
        // --------------------------------------------------
        // press 注册 / 一个 key
        // pressed 返回 / 是否处于按压状态
        // just_pressed 返回 / 当前桢为按压瞬间
        // release 注册 / 一个 key
        // released 返回 / 是否为释放状态
        // just_released 返回 / 当前桢为释放瞬间
        // --------------------------------------------------

        // 会持续大量的输出按压信息
        // if keys.pressed(*key) {
        //     println!("keys.pressed(*key)");
        // }

        //所以当前不是按压状态,就是释放的时候
        if !keys.pressed(*key) {
            println!("!keys.pressed(*key)");
            // 从实体上删除组件
            commands.entity(*entity).remove::<MyComponent>();
        }
        // 其实与!keys.pressed(*key)一样的结果
        // if keys.just_released(*key) {
        //     println!("keys.just_released(*key)");
        // }
    }

    //获得一个正好按下的 keycode ,用其值生成一个 包含 MyComponent 的 Entity
    for key in keys.get_just_pressed() {
        commands.spawn(MyComponent(*key));
    }
}
