//! This example illustrates how to react to component and resource changes.
//! 如何被动的检测到 component 与 resource 的变更
// migration: 0.16.x -> 0.17.x
// --features="track_change_detection" 更名为 "track_location"
//
// 2025-10-13T13:25:24.417022Z  WARN change_detection: Change detected!
//         -> value: Ref(MyComponent(0.0))
//         -> added: true
//         -> changed: true
//         -> changed by: examples/ecs/change_detection.rs:36:14      << change_component
// 2025-10-13T13:25:24.417063Z  WARN change_detection: Change detected!
//         -> value: Res(MyResource(0.0))
//         -> added: true
//         -> changed: true
//         -> changed by: examples/ecs/change_detection.rs:37:14
// 2025-10-13T13:25:24.515772Z  INFO change_detection: New value: MyComponent(0.0) 11v0
// 2025-10-13T13:25:26.548059Z  INFO change_detection: New value: MyComponent(1.0) 11v0
// 2025-10-13T13:25:26.548089Z  WARN change_detection: Change detected!
//         -> value: Ref(MyComponent(1.0))
//         -> added: false
//         -> changed: true
//         -> changed by: examples/ecs/change_detection.rs:50:23    << change_component_2

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                change_component,   // 更新 component
                change_component_2, // 功能同上，但检测时能区别出变更（函数/行号）等不同
                change_resource,    // 更新 resource
                change_detection,   // 检测报告
            ),
        )
        .run();
}

#[derive(Component, PartialEq, Debug)]
struct MyComponent(f32);

#[derive(Resource, PartialEq, Debug)]
struct MyResource(f32);

fn setup(mut commands: Commands) {
    // Note the first change detection log correctly points to this line because the component is
    // added. Although commands are deferred, they are able to track the original calling location.
    // 此处 "变更" 状态被定义为 add
    commands.spawn(MyComponent(0.0));
    commands.insert_resource(MyResource(0.0));
}

fn change_component(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    for (entity, mut component) in &mut query {
        if rand::rng().random_bool(0.1) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value: {new_component:?} {entity:?}");
            // Change detection occurs on mutable dereference, and does not consider whether or not
            // a value is actually equal. To avoid triggering change detection when nothing has
            // actually changed, you can use the `set_if_neq` method on any component or resource
            // that implements PartialEq.
            // 如果 component 原值与 new_component 新值不一致,使用新值
            component.set_if_neq(new_component);
        }
    }
}

/// This is a duplicate of the `change_component` system, added to show that change tracking can
/// help you find *where* your component is being changed, when there are multiple possible
/// locations.
fn change_component_2(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    for (entity, mut component) in &mut query {
        if rand::rng().random_bool(0.1) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value: {new_component:?} {entity:?}");
            component.set_if_neq(new_component);
        }
    }
}

/// Change detection concepts for components apply similarly to resources.
fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    if rand::rng().random_bool(0.1) {
        let new_resource = MyResource(time.elapsed_secs().round());
        info!("New value: {new_resource:?}");
        my_resource.set_if_neq(new_resource);
    }
}

/// Query filters like [`Changed<T>`] and [`Added<T>`] ensure only entities matching these filters
/// will be returned by the query.
///
/// Using the [`Ref<T>`] system param allows you to access change detection information, but does
/// not filter the query.
// 此处举例了 Query 查询 Component 的用法,
// 通常 Query 用于查询 Entity Query<Entity,...>
// 现在 Query<Ref<MyComponent>,Changed<MyComponent>> Changed 得到 change / add 两种状态
fn change_detection(
    changed_components: Query<Ref<MyComponent>, Changed<MyComponent>>,
    my_resource: Res<MyResource>,
) {
    for component in &changed_components {
        // By default, you can only tell that a component was changed.
        //
        // This is useful, but what if you have multiple systems modifying the same component, how
        // will you know which system is causing the component to change?
        warn!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}\n\t-> changed by: {}",
            component,
            component.is_added(), // 只在 setup 触发一次为 ture
            component.is_changed(),
            // If you enable the `track_change_detection` feature, you can unlock the `changed_by()`
            // method. It returns the file and line number that the component or resource was
            // changed in. It's not recommended for released games, but great for debugging!
            component.changed_by()
        );
    }

    if my_resource.is_changed() {
        warn!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}\n\t-> changed by: {}",
            my_resource,
            my_resource.is_added(),
            my_resource.is_changed(),
            my_resource.changed_by() // Like components, requires `track_change_detection` feature.
        );
    }
}
