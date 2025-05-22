use bevy::prelude::*;

use super::UIResource;

#[derive(Component, Debug)]
pub struct UILayoutStatistic;

// 巩固一下, &mut World 的使用写法
pub fn statistic(world: &mut World) -> Result {
    // 曾创建过的所有实体
    let total_count_entities = world.entities().total_count();
    // 当前使用中的实体
    let use_count_entities = world.entities().used_count();

    let Some(resource) = world.get_resource::<UIResource>() else {
        return Ok(());
    };
    let Some(entity) = resource.statistic else {
        return Ok(());
    };
    // 注意: 先要拿出一个可写的 EntityWorldMut 不能用链式调用
    let mut entity_mut = world.entity_mut(entity);
    // 再拿出可写的 Component
    let Some(mut text) = entity_mut.get_mut::<Text>() else {
        return Ok(());
    };

    text.0 = format!(
        "Total: {}\nUsed: {}",
        total_count_entities, use_count_entities
    );

    Ok(())
}
