use bevy::{platform::collections::HashSet, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Resource, Default)]
struct State {
    count: usize,
    generation: bool,
    concern_index: usize,
    concern_entity: Option<Entity>,
    ball_handle: Option<Handle<Mesh>>,
    // 绿色
    ball_material_normal: Option<Handle<ColorMaterial>>,
    // 白色
    ball_material_concern: Option<Handle<ColorMaterial>>,
    // 红色
    ball_material_contact: Option<Handle<ColorMaterial>>,
    // 黄色
    ball_material_prepare: Option<Handle<ColorMaterial>>,
    // 当 collider 碰撞不再接触后,我们需要恢复原来的颜色
    revert_collider: HashSet<Entity>,
}

#[derive(Component)]
struct ColliderIndex(usize);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());
    let state = State {
        concern_index: 100,
        ..default()
    };
    app.insert_resource(state);

    app.add_systems(Startup, (setup, ui, show_grid).chain());
    app.add_systems(Update, (create_entities, controls, concern_guard));
    app.run();
}

fn ui(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn concern_guard(
    mut commands: Commands,
    mut state: ResMut<State>,
    read_rapier: ReadRapierContext,
    time: Res<Time>,
) -> Result {
    let Some(entity) = state.concern_entity else {
        return Ok(());
    };
    let rapier_context = read_rapier.single()?;
    // 收集所有将会(已经)与 entity 发生碰撞的成对 collider
    let iter = rapier_context.contact_pairs_with(entity);
    let mut contact_count = 0;

    // 保存上次作用的 entity 准备进行交差恢复
    let pre_revert_collider = state.revert_collider.clone();
    state.revert_collider.clear();

    for contact_pair in iter {
        contact_count += 1;
        let Some(c1) = contact_pair.collider1() else {
            println!("Collider1 not found");
            continue;
        };
        let Some(c2) = contact_pair.collider2() else {
            println!("Collider2 not found");
            continue;
        };

        // 保存到恢复集合
        if c1 != entity {
            state.revert_collider.insert(c1);
        }
        if c2 != entity {
            state.revert_collider.insert(c2);
        }

        // 是否已经发生发生接触(contact)
        // iterator 中同样存在还未发生 contact 的 collider
        if contact_pair.has_any_active_contact() {
            // 所有已发生接触的设为红色
            let contact = state.ball_material_contact.clone().unwrap();
            let Some(concern) = state.concern_entity else {
                println!("Concern entity not found");
                return Ok(());
            };
            if c1 != concern {
                commands.entity(c1).insert(MeshMaterial2d(contact.clone()));
            }
            if c2 != concern {
                commands.entity(c2).insert(MeshMaterial2d(contact.clone()));
            }
        }
    }
    println!(
        "has contact ({}) {:?}",
        contact_count,
        time.elapsed_secs_wrapped(),
    );

    // 开始交差恢复
    // 上次记录的 entity 如果不在本次接触中,则恢复颜色
    let diff = pre_revert_collider
        .difference(&state.revert_collider)
        .collect::<Vec<_>>();
    let normal = state.ball_material_normal.clone().unwrap();
    for entity in diff {
        commands
            .entity(*entity)
            .insert(MeshMaterial2d(normal.clone()));
    }
    //println!("Contact pairs: {:?}", iter.count());
    Ok(())
}

fn controls(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &ColliderIndex)>,
    mut state: ResMut<State>,
    mut render_context: ResMut<DebugRenderContext>,
) {
    // manual sleeping
    if input.just_pressed(KeyCode::Space) {
        let previous = state.count.saturating_sub(100);
        println!("Sleeping: {}", previous);
        for (entity, index) in query {
            if index.0 < previous {
                // 因为 Sleeping 状态会被轻微碰撞激活,所以几乎没有意义
                // commands.entity(entity).insert(Sleeping {
                //     sleeping: true,
                //     ..default()
                // });
                commands.entity(entity).insert(RigidBody::Fixed);
            }
        }
    }

    // manual generation
    if input.just_pressed(KeyCode::KeyG) {
        state.generation = !state.generation;
        println!("Generation: {}", state.generation);
    }

    // manual render debug
    if input.just_pressed(KeyCode::KeyD) {
        render_context.enabled = !render_context.enabled;
        println!("RenderDebugSwitch: {}", render_context.enabled);
    }
}

//
fn create_entities(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<State>,
    mut text: Single<&mut Text>,
) -> Result {
    text.0 = format!(
        "RenderDebugSwitch: D\nRigidBodyFixed: Space\nGeneration: G\nEntitiesCount: {}\n",
        state.count
    );

    if !state.generation {
        return Ok(());
    }
    let x = time.elapsed_secs().sin() * 250.;
    state.count += 1;

    let Some(ball_handle) = state.ball_handle.clone() else {
        return Ok(());
    };

    let ball_material = if state.concern_index == state.count {
        state.ball_material_concern.clone().unwrap()
    } else {
        state.ball_material_normal.clone().unwrap()
    };

    let entity = commands
        .spawn((
            Mesh2d(ball_handle.clone()),
            MeshMaterial2d(ball_material),
            RigidBody::Dynamic,
            Collider::ball(5.),
            Transform::from_xyz(x, 300., 0.),
            ColliderIndex(state.count),
            // (性能优化) 阻尼可加快碰撞体进入休眠状态
            // Damping {
            //     linear_damping: 10.,
            //     angular_damping: 10.,
            // },
            // (性能优化) 休眠状态 *_threshold 过小会导致休眠困难
            // 而且有阈值上限
            // 并且会被再次激活
            Sleeping {
                normalized_linear_threshold: 1.,
                angular_threshold: 1.,
                sleeping: false,
            },
        ))
        .id();
    // 捕获关心的实体
    if state.count == state.concern_index {
        state.concern_entity = Some(entity);
    }

    Ok(())
}

// 造个大杯子
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State>,
) -> Result {
    commands.spawn(Camera2d);

    state.ball_handle = Some(meshes.add(Circle::new(5.)));
    state.ball_material_normal = Some(materials.add(Color::srgb_u8(0, 128, 0)));
    state.ball_material_concern = Some(materials.add(Color::WHITE));
    state.ball_material_contact = Some(materials.add(Color::srgb_u8(128, 0, 0)));
    state.ball_material_prepare = Some(materials.add(Color::srgb_u8(128, 128, 0)));

    // make a big cup
    let side_wall = Collider::cuboid(5., 300.);
    let bottom = Collider::cuboid(200., 5.);
    let rot = std::f32::consts::FRAC_PI_8;
    let vec = vec![
        (Vec2::new(-200., 0.), rot, side_wall.clone()),
        (Vec2::new(200., 0.), -rot, side_wall),
        (Vec2::new(0., -280.), 0., bottom),
    ];
    // generate a shape by cuboid
    for (pos, rot, collider) in vec.iter() {
        let Some(cuboid) = collider.raw.as_cuboid() else {
            return Ok(());
        };
        let shape = Rectangle::new(cuboid.half_extents.x * 2., cuboid.half_extents.y * 2.);
        commands.spawn((
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(pos.x, pos.y, 0.).with_rotation(Quat::from_rotation_z(*rot)),
        ));
    }
    // generate a compound collider
    let mug = Collider::compound(vec);
    commands.spawn((
        mug,
        Transform::from_xyz(0., 0., 0.),
        // 性能优化(不需要参与计算的 Collider 指定为 Fixed)
        RigidBody::Fixed,
    ));

    Ok(())
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,                   // 投影模式
            UVec2::new(64, 36),                     // 单元格数量
            Vec2::new(20., 20.),                    // 单元格大小
            LinearRgba::gray(0.05).with_alpha(0.2), // 网格颜色
        )
        .outer_edges();
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
