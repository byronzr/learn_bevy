#![allow(unsafe_code)]

//! This example show how you can create components dynamically, spawn entities with those components
//! as well as query for entities with those components.

// $ cargo run --example dynamic
//    Compiling learn_bevy v0.1.0 (/Users/byronzr/rProjects/learn_bevy)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
//      Running `target/debug/examples/dynamic`
//
// Commands:
//     comp, c   Create new components
//     spawn, s  Spawn entities
//     query, q  Query for entities
// Enter a command with no parameters for usage.
//
// > c CompA 3,CompB 2
// Component CompA created with id: 4/ComponentId(4) info: ComponentInfo { ... }
// Component CompB created with id: 5/ComponentId(5) info: ComponentInfo { ... }
//
// > s CompA 7 8 9,CompB
// Entity spawned with id: 0v1#4294967296
//
// > q &CompA
// 0v1#4294967296: CompA: [7, 8, 9]
//
// > q &CompB
// 0v1#4294967296: CompB: [0, 0]
//
// > q &mut CompB
// 0v1#4294967296: CompB: [1, 1]
//
// >

use std::{alloc::Layout, collections::HashMap, io::Write, ptr::NonNull};

use bevy::{
    ecs::{
        component::{
            ComponentCloneBehavior, ComponentDescriptor, ComponentId, ComponentInfo, StorageType,
        },
        query::QueryData,
        world::FilteredEntityMut,
    },
    prelude::*,
    ptr::{Aligned, OwningPtr},
    //utils::HashMap,
};

const PROMPT: &str = "
Commands:
    comp, c   Create new components
    spawn, s  Spawn entities
    query, q  Query for entities
Enter a command with no parameters for usage.";

const COMPONENT_PROMPT: &str = "
comp, c   Create new components
    Enter a comma separated list of type names optionally followed by a size in u64s.
    e.g. CompA 3, CompB, CompC 2";

const ENTITY_PROMPT: &str = "
spawn, s  Spawn entities
    Enter a comma separated list of components optionally followed by values.
    e.g. CompA 0 1 0, CompB, CompC 1";

const QUERY_PROMPT: &str = "
query, q  Query for entities
    Enter a query to fetch and update entities
    Components with read or write access will be displayed with their values
    Components with write access will have their fields incremented by one

    Accesses: 'A' with, '&A' read, '&mut A' write
    Operators: '||' or, ',' and, '?' optional

    e.g. &A || &B, &mut C, D, ?E";

fn main() {
    let mut world = World::new();
    // 获得一个按行读取的迭代器
    let mut lines = std::io::stdin().lines();
    let mut component_names = HashMap::<String, ComponentId>::new();
    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();

    println!("{PROMPT}");
    loop {
        print!("\n> ");
        let _ = std::io::stdout().flush();

        // 读取一行字符
        let Some(Ok(line)) = lines.next() else {
            return;
        };

        // 如果为空退出程序
        if line.is_empty() {
            return;
        };

        // 根据单字符进入下一级菜单
        // split_once 依次传入 char
        // 如果 c 为空白字符返回 ture,那么 splite_once,则以 c 为分割符,将 line分割为 (first,rest)
        // 这样的好处是在于 is_whitespace 可以判断空格,TAB等多种类型
        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
            match &line.chars().next() {
                Some('c') => println!("{COMPONENT_PROMPT}"),
                Some('s') => println!("{ENTITY_PROMPT}"),
                Some('q') => println!("{QUERY_PROMPT}"),
                _ => println!("{PROMPT}"),
            }
            continue;
        };

        //
        match &first[0..1] {
            // 创建 component
            "c" => {
                rest.split(',').for_each(|component| {
                    let mut component = component.split_whitespace();
                    let Some(name) = component.next() else {
                        return;
                    };
                    // 初始化一个用于创建 component.Layout 的 size 大小
                    let size = match component.next().map(str::parse) {
                        Some(Ok(size)) => size,
                        _ => 0,
                    };
                    // Register our new component to the world with a layout specified by it's size
                    // SAFETY: [u64] is Send + Sync
                    // 注册一个 component
                    let id = world.register_component_with_descriptor(unsafe {
                        ComponentDescriptor::new_with_layout(
                            name.to_string(),
                            StorageType::Table,
                            Layout::array::<u64>(size).unwrap(),
                            None,
                            true,                            // since 0.17.0
                            ComponentCloneBehavior::Default, // since 0.17.0
                        )
                    });
                    let Some(info) = world.components().get_info(id) else {
                        return;
                    };
                    component_names.insert(name.to_string(), id);
                    component_info.insert(id, info.clone());
                    println!(
                        "Component {} created with id: {:?}/{:?} info: {:?}",
                        name,
                        id.index(), // 4
                        id,         // ComponentId(4)
                        info        // TMI
                    );
                });
            }
            // 创建 Entity
            "s" => {
                // 准备创建 Entity 的容器,解析完成字符串后,统一创建
                let mut to_insert_ids = Vec::new();
                let mut to_insert_data = Vec::new();
                // 解析 Entity 创建要求与内容
                rest.split(',').for_each(|component| {
                    let mut component = component.split_whitespace();
                    // 获得 component 的名字
                    let Some(name) = component.next() else {
                        return;
                    };

                    // Get the id for the component with the given name
                    // 从名字获得关联 component id
                    let Some(&id) = component_names.get(name) else {
                        println!("Component {name} does not exist");
                        return;
                    };

                    // Calculate the length for the array based on the layout created for this component id
                    // 获得 component layout 的大小,预防 Entity 创建时大小与原 component 大小不一致
                    let info = world.components().get_info(id).unwrap();
                    let len = info.layout().size() / size_of::<u64>();
                    // 分解命令行创建 Entity 输入的 info 其它内容,
                    // 保证每个 whitespace 分割的 str 都可以被 parse 为一个 u64
                    // 获得一个 Vec<u64>, 确保内容以 Entity 创建内容为主,如果少于 Component 预设大小以0填充
                    let mut values: Vec<u64> = component
                        .take(len)
                        .filter_map(|value| value.parse::<u64>().ok())
                        .collect();
                    values.resize(len, 0);

                    // Collect the id and array to be inserted onto our entity
                    to_insert_ids.push(id);
                    to_insert_data.push(values);
                });

                // 开始创建 Entity
                let mut entity = world.spawn_empty();

                // Construct an `OwningPtr` for each component in `to_insert_data`
                // 将 comonent 的 Vec<u64> 类型,转换成指针迭代器 Vec<OwningPtr>
                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);

                // SAFETY:
                // - Component ids have been taken from the same world
                // - Each array is created to the layout specified in the world
                // 将 component id 集合与 component info 集合,嵌入 entity
                unsafe {
                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
                }

                println!("Entity spawned with id: {:?}", entity.id());
            }
            // 创建动态查询
            "q" => {
                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
                parse_query(rest, &mut builder, &component_names);
                let mut query = builder.build();

                // 逐一遍历 query 结果集中的每个 Entity
                query.iter_mut(&mut world).for_each(|filtered_entity| {
                    // 屏蔽编译弃用警告
                    #[allow(deprecated)]
                    let terms = filtered_entity
                        .access()
                        // 获得 entity 的 component id 集合迭代器
                        // .component_reads_and_writes()
                        .try_iter_component_access()
                        // 无视 tuple 的第二个值,只用 iter
                        // .0
                        .unwrap() // since 0.17.0
                        // .map(|id| {
                        //     // 获得一个 component 指针
                        //     let ptr = filtered_entity.get_by_id(id).unwrap();
                        //     // 从 comonent_info(全局) 中获得 info 信息
                        //     let info = component_info.get(&id).unwrap();
                        //     let len = info.layout().size() / size_of::<u64>();
                        // since 0.17.0
                        .map(|component_access| {
                            let id = *component_access.index();
                            let ptr = filtered_entity.get_by_id(id).unwrap();
                            let info = component_info.get(&id).unwrap();
                            let len = info.layout().size() / size_of::<u64>();

                            // SAFETY:
                            // - All components are created with layout [u64]
                            // - len is calculated from the component descriptor
                            // 获得 component info 的可写引用
                            let data = unsafe {
                                std::slice::from_raw_parts_mut(
                                    ptr.assert_unique().as_ptr().cast::<u64>(),
                                    len,
                                )
                            };

                            // If we have write access, increment each value once
                            // 如果 query 是一个可写查询,将内容逐一增加1
                            if filtered_entity.access().has_component_write(id) {
                                data.iter_mut().for_each(|data| {
                                    *data += 1;
                                });
                            }

                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    println!("{:?}: {}", filtered_entity.id(), terms);
                });
            }
            _ => continue,
        };
    }
}

// Constructs `OwningPtr` for each item in `components`
// By sharing the lifetime of `components` with the resulting ptrs we ensure we don't drop the data before use
fn to_owning_ptrs(components: &mut [Vec<u64>]) -> Vec<OwningPtr<'_, Aligned>> {
    components
        .iter_mut()
        .map(|data| {
            let ptr = data.as_mut_ptr();
            // SAFETY:
            // - Pointers are guaranteed to be non-null
            // - Memory pointed to won't be dropped until `components` is dropped
            unsafe {
                let non_null = NonNull::new_unchecked(ptr.cast());
                OwningPtr::new(non_null)
            }
        })
        .collect()
}

fn parse_term<Q: QueryData>(
    str: &str,
    builder: &mut QueryBuilder<Q>,
    components: &HashMap<String, ComponentId>,
) {
    let mut matched = false;
    let str = str.trim();
    match str.chars().next() {
        // Optional term
        Some('?') => {
            builder.optional(|b| parse_term(&str[1..], b, components));
            matched = true;
        }
        // Reference term
        Some('&') => {
            let mut parts = str.split_whitespace();
            let first = parts.next().unwrap();
            if first == "&mut" {
                if let Some(str) = parts.next() {
                    if let Some(&id) = components.get(str) {
                        builder.mut_id(id);
                        matched = true;
                    }
                };
            } else if let Some(&id) = components.get(&first[1..]) {
                builder.ref_id(id);
                matched = true;
            }
        }
        // With term
        Some(_) => {
            if let Some(&id) = components.get(str) {
                builder.with_id(id);
                matched = true;
            }
        }
        None => {}
    };

    if !matched {
        println!("Unable to find component: {str}");
    }
}

fn parse_query<Q: QueryData>(
    str: &str,
    builder: &mut QueryBuilder<Q>,
    components: &HashMap<String, ComponentId>,
) {
    let str = str.split(',');
    str.for_each(|term| {
        let sub_terms: Vec<_> = term.split("||").collect();
        if sub_terms.len() == 1 {
            parse_term(sub_terms[0], builder, components);
        } else {
            builder.or(|b| {
                sub_terms
                    .iter()
                    .for_each(|term| parse_term(term, b, components));
            });
        }
    })
}
