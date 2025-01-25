//! Creates a hierarchy of parents and children entities.

use std::f32::consts::*;

use bevy::{color::palettes::css::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

// 生成三个 sprite
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let texture = asset_server.load("branding/icon.png");

    // Spawn a root entity with no parent
    // 相当于一个 Root Entity (白色)
    let parent = commands
        .spawn((
            Sprite::from_image(texture.clone()),
            // 基于父节点(screen?canvas?)的形变和位移
            Transform::from_scale(Vec3::splat(0.75)),
        ))
        // With that entity as a parent, run a lambda that spawns its children
        // 为当前 Entity 创建 Children Entity (蓝色)
        .with_children(|parent| {
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn((
                // 基于父节点(parent entity)的形变和位移(右)
                Transform::from_xyz(250.0, 0.0, 0.0).with_scale(Vec3::splat(0.75)),
                Sprite {
                    image: texture.clone(),
                    color: BLUE.into(),
                    ..default()
                },
            ));
        })
        // Store parent entity for next sections
        // 除了以 with_children 方式创建 Entity 可能也会使用 commands.entity(..) 进行嵌套
        // 在同一 system 中使用刚刚创建好 entity 进行操,那么 id() 就十分必要了.
        // 相当于提前取个票,业务办理银行自已安排时间顺序处理
        .id();

    // Another way is to use the add_child function to add children after the parent
    // entity has already been spawned.
    // 这是一种非链式调用的 children 创建方式,同样 id 也需要显式获取
    let child = commands
        .spawn((
            Sprite {
                image: texture,
                color: LIME.into(),
                ..default()
            },
            // 上
            Transform::from_xyz(0.0, 250.0, 0.0).with_scale(Vec3::splat(0.75)),
        ))
        .id();

    // Add child to the parent.
    commands.entity(parent).add_child(child);
}

// A simple system to rotate the root entity, and rotate all its children separately
fn rotate(
    mut commands: Commands,
    time: Res<Time>,
    // Children 不是一个 Component 所以无法放在 With 中
    // 但在 Query 的第一个参数中,同样可以作为一个限制,
    // 本例中,符合条件(拥有Sprite component 与 &Children ) 的 Entity,只有一个
    mut parents_query: Query<(Entity, &Children), With<Sprite>>, // only root
    mut transform_query: Query<&mut Transform, With<Sprite>>,    // all entity
) {
    for (parent, children) in &mut parents_query {
        // 因为 Entity 作了嵌套,每个 children 的 transform 都会受 parent 影响
        // parent 每次自旋都会联动所有 children
        if let Ok(mut transform) = transform_query.get_mut(parent) {
            transform.rotate_z(-PI / 2. * time.delta_secs());
        }

        // To iterate through the entities children, just treat the Children component as a Vec
        // Alternatively, you could query entities that have a Parent component
        // 而此处,只是完成了 children 的自旋转
        for child in children {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotate_z(PI * time.delta_secs());
            }
        }

        // To demonstrate removing children, we'll remove a child after a couple of seconds.
        // 2 秒后,删除一个 children entity
        if time.elapsed_secs() >= 2.0 && children.len() == 2 {
            let child = children.last().unwrap();
            commands.entity(*child).despawn_recursive();
        }

        // 4 秒后,删除 parent 与基联动的 所有 children
        if time.elapsed_secs() >= 4.0 {
            // This will remove the entity from its parent's list of children, as well as despawn
            // any children the entity has.
            commands.entity(parent).despawn_recursive();
        }
    }
}
