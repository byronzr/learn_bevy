//! This example illustrates how to react to component and resource changes.
//! 如何被动的检测到 component 与 resource 的变更
//! 运行这个例子需要 --features="track_change_detection"

use bevy::prelude::*;
use rand::Rng;

fn main() {
    std::env::set_var("NO_COLOR", "1");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                change_component,   // 更新 component
                change_component_2, // 与同上功能一模一样,它存在的意义在于 changed_by() 可以检测到具体的是哪个文件哪个函数
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
        if rand::thread_rng().gen_bool(0.1) {
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
        if rand::thread_rng().gen_bool(0.1) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value: {new_component:?} {entity:?}");
            component.set_if_neq(new_component);
        }
    }
}

/// Change detection concepts for components apply similarly to resources.
fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    if rand::thread_rng().gen_bool(0.1) {
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
