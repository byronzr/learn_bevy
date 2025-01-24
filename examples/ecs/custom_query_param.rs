//! This example illustrates the usage of the [`QueryData`] derive macro, which allows
//! defining custom query and filter types.
//!
//! While regular tuple queries work great in most of simple scenarios, using custom queries
//! declared as named structs can bring the following advantages:
//! - They help to avoid destructuring or using `q.0, q.1, ...` access pattern.
//! - Adding, removing components or changing items order with structs greatly reduces maintenance
//!   burden, as you don't need to update statements that destructure tuples, care about order
//!   of elements, etc. Instead, you can just add or remove places where a certain element is used.
//! - Named structs enable the composition pattern, that makes query types easier to re-use.
//! - You can bypass the limit of 15 components that exists for query tuples.
//!
//! For more details on the [`QueryData`] derive macro, see the trait documentation.

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};
use std::fmt::Debug;

fn main() {
    App::new()
        .add_systems(Startup, spawn)
        .add_systems(
            Update,
            (
                print_components_read_only,
                print_components_iter_mut,
                print_components_iter,
                print_components_tuple,
            )
                .chain(),
        )
        .run();
}

#[derive(Component, Debug)]
struct ComponentA;
#[derive(Component, Debug)]
struct ComponentB;
#[derive(Component, Debug)]
struct ComponentC;
#[derive(Component, Debug)]
struct ComponentD;
#[derive(Component, Debug)]
struct ComponentZ;

// QueryData 的目的,是集成查询结果到一个自定义结构体.
// 所以,如果 Query 中的 Entity 不包含(非Option集成)某个 Component 时,就不会有结果
#[derive(QueryData)]
#[query_data(derive(Debug))]
struct ReadOnlyCustomQuery<T: Component + Debug, P: Component + Debug> {
    entity: Entity,
    a: &'static ComponentA,
    b: Option<&'static ComponentB>,
    nested: NestedQuery,
    optional_nested: Option<NestedQuery>,
    optional_tuple: Option<(&'static ComponentB, &'static ComponentZ)>,
    // z: &'static ComponentZ, // 如果你使用了这个非 Option 集成,你将得不到 Query 结果
    z: Option<&'static ComponentZ>, // ComponentZ 从未被 add ,在 world 不存在 entity 拥有 ComponentZ 如果不使用 Option<> 将不会有任何结果输出
    generic: GenericQuery<T, P>,
    empty: EmptyQuery,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct ReadOnlyCustomQuery2<T: Component + Debug, P: Component + Debug> {
    entity: Entity,
    generic: GenericQuery<T, P>,
}

/// 只读查询案例
fn print_components_read_only(
    query: Query<
        ReadOnlyCustomQuery<ComponentC, ComponentD>,
        CustomQueryFilter<ComponentC, ComponentD>,
    >,
) {
    println!("Print components (read_only):");
    for e in &query {
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Nested: {:?}", e.nested);
        println!("Optional nested: {:?}", e.optional_nested);
        println!("Optional tuple: {:?}", e.optional_tuple);
        println!("Generic: {:?}", e.generic);
        println!("empty: {:?}", e.empty);
    }
    println!();
}

/// If you are going to mutate the data in a query, you must mark it with the `mutable` attribute.
///
/// The [`QueryData`] derive macro will still create a read-only version, which will be have `ReadOnly`
/// suffix.
/// Note: if you want to use derive macros with read-only query variants, you need to pass them with
/// using the `derive` attribute.
// CustomQueryReadOnlyItem<'_, _, _> // 只读版本的 item
// CustomQueryItem<'_, _, _> // 可写版本的 item
// 这两个item 都由宏完成定义
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct CustomQuery<T: Component + Debug, P: Component + Debug> {
    entity: Entity,
    a: &'static mut ComponentA,
    b: Option<&'static mut ComponentB>,
    nested: NestedQuery,
    optional_nested: Option<NestedQuery>,
    optional_tuple: Option<(NestedQuery, &'static mut ComponentZ)>,
    generic: GenericQuery<T, P>,
    empty: EmptyQuery,
}

// This is a valid query as well, which would iterate over every entity.
#[derive(QueryData)]
#[query_data(derive(Debug))]
struct EmptyQuery {
    empty: (),
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct NestedQuery {
    c: &'static ComponentC,
    d: Option<&'static ComponentD>,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct GenericQuery<T: Component, P: Component> {
    generic: (&'static T, &'static P),
}

// 所有的结构属性,默认都是 and 联合,所以一但有一个条件不满足,将会没有结果
#[derive(QueryFilter)]
struct CustomQueryFilter<T: Component, P: Component> {
    _c: With<ComponentC>,
    _d: With<ComponentD>,
    // 如果不将 Added 加入到 Or 条件中时,那么 Query 只会得到一次结果
    // _add: Added<ComponentC>,
    _or: Or<(Added<ComponentC>, Changed<ComponentD>, Without<ComponentZ>)>,
    _generic_tuple: (With<T>, With<P>),
    // 取消注释,本例中永远无法满足 ComponentZ 条件
    // _generic_tuple2: (With<T>, With<P>, With<ComponentZ>),
}

/// A,B,C,D 被生成到一个 Entity 中,但不包括 Z
fn spawn(mut commands: Commands) {
    commands.spawn((ComponentA, ComponentB, ComponentC, ComponentD));
}

/// 当使用可写版本的 item 时,需要有两个地方加 mut
fn print_components_iter_mut(
    mut query: Query<
        CustomQuery<ComponentC, ComponentD>,
        CustomQueryFilter<ComponentC, ComponentD>,
    >,
) {
    println!("Print components (iter_mut):");
    for e in &mut query {
        // Re-declaring the variable to illustrate the type of the actual iterator item.
        let e: CustomQueryItem<'_, _, _> = e;
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Optional nested: {:?}", e.optional_nested);
        println!("Optional tuple: {:?}", e.optional_tuple);
        println!("Nested: {:?}", e.nested);
        println!("Generic: {:?}", e.generic);
    }
    println!();
}

/// 这是一个只读版本
fn print_components_iter(
    query: Query<CustomQuery<ComponentC, ComponentD>, CustomQueryFilter<ComponentC, ComponentD>>,
) {
    println!("Print components (iter):");
    for e in &query {
        // Re-declaring the variable to illustrate the type of the actual iterator item.
        let e: CustomQueryReadOnlyItem<'_, _, _> = e;
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Nested: {:?}", e.nested);
        println!("Generic: {:?}", e.generic);
    }
    println!();
}

type NestedTupleQuery<'w> = (&'w ComponentC, &'w ComponentD);
type GenericTupleQuery<'w, T, P> = (&'w T, &'w P);

/// 一种常规传统的 Query 方式,
/// 可以看到 Query 的复杂度集中到了 function 参数中,不易于阅读
fn print_components_tuple(
    query: Query<
        (
            Entity,
            &ComponentA,
            &ComponentB,
            NestedTupleQuery,
            GenericTupleQuery<ComponentC, ComponentD>,
        ),
        (
            With<ComponentC>,
            With<ComponentD>,
            Or<(Added<ComponentC>, Changed<ComponentD>, Without<ComponentZ>)>,
        ),
    >,
) {
    println!("Print components (tuple):");
    for (entity, a, b, nested, (generic_c, generic_d)) in &query {
        println!("Entity: {entity:?}");
        println!("A: {a:?}");
        println!("B: {b:?}");
        println!("Nested: {:?} {:?}", nested.0, nested.1);
        println!("Generic: {generic_c:?} {generic_d:?}");
    }
}
