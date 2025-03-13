use std::f32::consts::PI;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashSet};
use rand::{rng, seq::IndexedRandom};

use bevy_rapier2d::prelude::*;

// use std::io::{self, Write};

pub mod env;
pub mod utils;
use env::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // bevy native picking_backend
    //app.add_plugins(MeshPickingPlugin);

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(RapierDebugRenderPlugin {
        default_collider_debug: ColliderDebug::NeverRender,
        style: DebugRenderStyle {
            subdivisions: 1,
            border_subdivisions: 1,
            ..default()
        },
        ..default()
    });

    app.init_state::<GameState>();
    app.init_resource::<PretreatSet>();
    app.init_resource::<WorldMap>();

    // init player info
    let inf = PlayerInfo {
        coordiate: (7, 14),
        movement_range: 1, // 一格行走,是不需要循路的
        sight_range: 3,
        ..Default::default()
    };
    app.insert_resource(inf);
    app.add_systems(OnEnter(GameState::Loading), load_textures);
    app.add_systems(
        Update, // after 确保 LoadTexture 资源被放入
        (
            check_textures.after(load_textures),
            clear_fow.after(render_map),
            mouse_action,
        ),
    );
    app.add_systems(PostUpdate, intersection_test_with_rapier2d);

    app.add_systems(
        RunFixedMainLoop,
        animate_player.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
    );

    // 构建世界数据
    app.add_systems(OnEnter(GameState::GenerateWorld), generate_map_data);
    // 渲染世界(包括玩家)
    app.add_systems(OnEnter(GameState::InGame), render_map);
    app.run();
}

// 测试碰撞
fn intersection_test_with_rapier2d(
    mut events: EventReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    // 文档中的 ReadDefaultRapierContext已经不存在了,这里使用 ReadRapierContext
    rapier_context: ReadRapierContext,
    mut fow_query: Query<(Entity, &FowCoor)>,
    mut command: Commands,
) {
    // 最后一个鼠标移动事件
    let Some(event) = events.read().last() else {
        return;
    };
    // 转换坐标系
    let (camera, camera_transform) = *camera;
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.position) else {
        return;
    };
    let filter = QueryFilter::default();

    // ReadRapierContext 并没有 intersection 的一系列方法,需要使用 single
    // 得到 RapierContext
    let context = rapier_context.single();
    context.intersections_with_point(world_position, filter, |entity| {
        // 利用调试 Render 显示测试区域,
        // 没有实际的开发上的意义,commands,是有缓存的,并不能每次都将 ColliderDebug设置为 NeverRender
        for (id, fow) in &mut fow_query {
            if id == entity {
                println!("fow {:?}", fow);
                command.entity(entity).insert(ColliderDebug::AlwaysRender);
            } else {
                command.entity(entity).insert(ColliderDebug::NeverRender);
            }
        }
        true
    });
}

// 清理迷雾
fn clear_fow(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &FowCoor, &mut FowLevel)>,
    player_info: Res<PlayerInfo>,
    mut world_map: ResMut<WorldMap>,
) {
    // 初始化 fow_range
    if world_map.fow_range.is_none() {
        utils::load_fow(&mut world_map);
    }
    // 迷雾差值数组
    let fow = world_map.fow_range.as_ref().unwrap();

    let mut reachable_set = HashSet::new();

    for (mut material, fow_coor, fow_level) in query.iter_mut() {
        // 当前扫描坐标与玩家坐标的差值
        let difference_value = (
            fow_coor.0 as i32 - player_info.coordiate.0 as i32,
            fow_coor.1 as i32 - player_info.coordiate.1 as i32,
        );

        // 按玩家所在奇偶行取不同的范围
        let Some(range) = fow.range.get(player_info.coordiate.1 % 2) else {
            continue;
        };

        for (level, range_set) in range.iter().enumerate() {
            // 超出玩家视野,不进行检测
            if level >= player_info.sight_range {
                break;
            }

            // 当前坐标包含在差值数组中,则进行处理
            if range_set.contains(&difference_value) {
                // 打开迷雾
                if level < fow_level.0 {
                    //sprite.color = Color::WHITE.with_alpha(0.);
                    material.0 = world_map.fow_disappear[level].clone();
                }
                // 收集可到达区域
                if level > 0 && level <= player_info.movement_range {
                    reachable_set.insert((fow_coor.0, fow_coor.1));
                }
            }
        }
    }
    // 更新可到达区域
    if world_map.reachable_coordiate_set != reachable_set {
        world_map.reachable_coordiate_set = reachable_set;
    }
}

// 玩家动画切换与移动
fn animate_player(
    time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PlayerTimer,
        &mut Sprite,
        &AnimationIndices,
        &mut Transform,
        &mut PlayerState,
    )>,
    mut player_info: ResMut<PlayerInfo>,
    pretreat: Res<PretreatSet>,
    mut world_map: ResMut<WorldMap>,
) {
    let Ok((mut timer, mut sprite, indices, mut transform, mut state)) = query.get_single_mut()
    else {
        return;
    };
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let to_direction = player_info.destination - transform.translation.truncate();
    let distance = to_direction.length();
    let speed = 100.;

    // Walk
    if distance > 1. {
        // 找到目标方向,不需要转换
        let front = to_direction / distance;
        let step = speed * time.delta_secs() * front;
        transform.translation += step.extend(0.);
        if state.set_if_neq(PlayerState::Walk) {
            *sprite = pretreat
                .player
                .get(&PlayerState::Walk)
                .unwrap()
                .sprite
                .clone();
            // 确定朝向
            player_info.flipping = if to_direction.x < 0. { true } else { false }
        }
        let Some(atlas) = &mut sprite.texture_atlas else {
            return;
        };
        atlas.index = (atlas.index + 1) % (indices.walk + 1);
        sprite.flip_x = player_info.flipping;
        return;
    }
    // Idle
    if state.set_if_neq(PlayerState::Idle) {
        *sprite = pretreat
            .player
            .get(&PlayerState::Idle)
            .unwrap()
            .sprite
            .clone();
        // 接近目地后,完全赋值,使玩家与六边形对齐
        transform.translation = player_info.destination.extend(PLAYER_LAYER);
        // 清除 world_map.destination_coordiate,让鼠标可再次进行点击
        // 并且玩家更新到达坐标
        if let Some(coor) = world_map.destination_coordiate.take() {
            player_info.coordiate = coor;
        }
    }
    let Some(atlas) = &mut sprite.texture_atlas else {
        return;
    };
    atlas.index = (atlas.index + 1) % (indices.idle + 1);

    sprite.flip_x = player_info.flipping;
}

// 鼠标点击与移动事件
fn mouse_action(
    mut events: EventReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Sprite, &Transform, &TerrainMarker)>,
    input: Res<ButtonInput<MouseButton>>,
    mut player_info: ResMut<PlayerInfo>,
    mut world_map: ResMut<WorldMap>,
) {
    // 还在移动中,不显示颜色,也不允许点击
    if world_map.destination_coordiate.is_some() {
        return;
    }
    // 最后一个鼠标移动事件
    let Some(event) = events.read().last() else {
        return;
    };
    // 转换坐标系
    let (camera, camera_transform) = *camera;
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.position) else {
        return;
    };

    for (mut terrain, transform, marker) in &mut query {
        // 当前坐标是否存在于六边形内
        // 不在六边形内恢复颜色
        if !utils::point_in_flat_top_hexagon(
            world_position,
            transform.translation.truncate(),
            HEXAGON_SIDE_LENGTH,
        ) {
            terrain.color = Color::WHITE;
            continue;
        }
        // 当前坐标
        let coor = (marker.0, marker.1);

        // 不可到达,显示红色
        if !world_map.reachable_coordiate_set.contains(&coor) {
            terrain.color = Color::srgba(1., 0., 0., 0.5);
            continue;
        }
        // 可到达,显示绿色
        terrain.color = Color::srgba(0., 1., 0., 0.5);

        // released 事件,在这个场景中更符合操作习惯
        if input.just_released(MouseButton::Left) {
            player_info.destination = transform.translation.truncate();
            world_map.destination_coordiate = Some(coor);
        }
    }
}

// 渲染地图(包括玩家)
fn render_map(
    asset_server: Res<AssetServer>,
    mut world_map: ResMut<WorldMap>,
    mut commands: Commands,
    pretreat: Res<PretreatSet>,
    mut player_info: ResMut<PlayerInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    world_map.fow_appear = material.add(Color::srgba(0., 0., 0., 1.));
    for level in 0..player_info.sight_range {
        world_map.fow_disappear.push(material.add(Color::srgba(
            0.,
            0.,
            0.,
            level as f32 / player_info.sight_range as f32,
        )));
    }
    for (&(col, row), tm) in world_map.map.iter() {
        // 地形
        let Some(ei) = tm.terrain.clone() else {
            // 地形是必须的
            return;
        };

        // 初始化玩家起始点
        if (col, row) == player_info.coordiate {
            player_info.destination = tm.position;
        }
        let (scx, scy) = player_info.coordiate;

        commands.spawn((
            ei.sprite,
            Transform::from_translation(tm.position.extend(TERRAIN_LAYER)),
            TerrainMarker(col, row),
        ));

        // 建筑
        if let Some(building) = tm.building.clone() {
            commands.spawn((
                building.sprite,
                Transform::from_translation(tm.position.extend(BUILDING_LAYER)),
            ));
        }

        // NPC
        if let Some(npc) = tm.npc.clone() {
            commands.spawn((
                npc.sprite,
                Transform::from_translation(tm.position.extend(NPC_LAYER)),
            ));
        };

        // fow
        let mut transform = Transform::from_translation(tm.position.extend(FOW_LAYER));
        // 参数写的是 angle 但实际上是 radian
        //transform.rotate_local_z(90. * PI / 180.);
        transform.rotation = Quat::from_rotation_z(90. * PI / 180.);
        let shape = RegularPolygon::new(20., 6);
        let points = shape.vertices(20.).into_iter().collect::<Vec<Vec2>>();

        let collider = Collider::convex_hull(&points).expect("Failed to create convex hull");

        commands
            .spawn((
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(world_map.fow_appear.clone()),
                FowCoor(col, row),
                FowLevel(99),
                transform,
                collider,
            ))
            .with_children(|parent| {
                // 坐标
                let mut transform =
                    Transform::from_translation(Vec3::new(0., 0., COORDIANTE_LAYER));
                transform.rotate_z(-90. * PI / 180.);
                parent.spawn((
                    // 差值坐标
                    Text2d(format!(
                        "{},{}",
                        col as i32 - scx as i32,
                        row as i32 - scy as i32
                    )),
                    // 原始坐标
                    //Text2d(format!("{},{}", col, row)),
                    TextFont {
                        font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                        font_size: 12.0,
                        ..default()
                    },
                    transform,
                    Visibility::Hidden,
                ));
            })
            // 在启用 MeshPickingPlugin 后,Mesh2d 就能触发 Pcking 事件,
            // 并能很好的控制在自定义的范围之内,这将大大简化 <pointer on area> 的判断
            // 但它现在可能会出现性能问题,所以官方默认状态下是没有加入到 DefaultPlugins 中的,
            // 而需要单独加入
            .observe(observ_regularpolygon);
    }

    // Player
    if let Some(idle) = pretreat.player.get(&PlayerState::Idle) {
        let mut transform =
            Transform::from_translation(player_info.destination.extend(PLAYER_LAYER));
        transform.scale = Vec3::splat(0.75);
        commands.spawn((
            idle.sprite.clone(),
            AnimationIndices { idle: 5, walk: 9 },
            PlayerTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
            PlayerState::Idle,
            transform,
        ));
    }
}

// mesh2d 没有效果
// (但是如果启用了 MeshPickingPlugin,则可以使用,但是会有性能问题)
// Mesh: this is a naive raycast against the full mesh.
// If you run into performance problems here,
// you should use simplified meshes and an acceleration data structure like a BVH to speed this up.
// As a result, this functionality is currently disabled by default.
// It can be enabled by adding the MeshPickingPlugin.
fn observ_regularpolygon(trigger: Trigger<Pointer<Move>>) {
    println!("move {:?}", trigger);
}

// 集中加载资源
fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pretreat: ResMut<PretreatSet>,
    win: Single<&Window>,
) {
    info!("loading... textures");
    commands.insert_resource(LoadTexture(asset_server.load_folder("textures")));
    commands.spawn(Camera2d);
    pretreat.window_size = Vec2::new(win.physical_width() as f32, win.physical_height() as f32);
}

// 进行资源预处理
// (应当是在数据库读取资产信息,交由创建世界资源)
fn check_textures(
    texture: Res<LoadTexture>,
    mut pretreat: ResMut<PretreatSet>,
    mut event_reader: EventReader<AssetEvent<LoadedFolder>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut textures: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.read() {
        if event.is_loaded_with_dependencies(&texture.0) {
            // 可用 sprite 进行预处理
            utils::init_pretreat(&mut textures, &asset_server, &mut pretreat);
            // 推进状态生成地图
            info!("next state (GenerateWorld)");
            next_state.set(GameState::GenerateWorld);
        }
    }
}

// 生成地图数据
fn generate_map_data(
    mut world_map: ResMut<WorldMap>,
    pretreat: Res<PretreatSet>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 假设没有存档
    let has_save = false;
    if !has_save {
        random_data(&mut world_map, &pretreat);
    }

    // 进入游戏
    next_state.set(GameState::InGame);
}

// 随机地图数据
fn random_data(world_map: &mut WorldMap, pretreat: &PretreatSet) {
    let mut rng = rng();
    let center = pretreat.window_size / 2.;
    // 起始左上角 = 在上角坐标 + (地块一半+整体偏移量)
    let topleft = Vec2::new(-center.x, center.y)
        + (Vec2::splat(HEXAGON_HALF_SIZE) + MAP_OFFSET) * Vec2::new(1., -1.);

    let coordiate_set = world_map.init_coordiate_combined();
    for (col, row) in coordiate_set.into_iter() {
        // 随机地形
        let Some(Some(terrain)) = pretreat.terrain.choose(&mut rng).cloned() else {
            error!("can't choose terrain.");
            return;
        };

        // 随机建筑
        let building = if terrain.name.eq("lake") {
            None
        } else {
            let Some(v) = pretreat.building.choose(&mut rng).cloned() else {
                return;
            };
            v
        };

        // 因为六边形是奇偶行错开的,所以需要计算偏移
        let offset = if row % 2 == 0 {
            // 偶数行无多余偏移
            0.
        } else {
            // 奇数行偏移 = 间隔 + 边长 + 侧边宽
            HEXAGON_GAP + HEXAGON_SIDE_LENGTH + HEXAGON_SIDE_WIDTH
        };
        let tm = TileMap {
            position: topleft
                + Vec2::new(
                    // x = (宽度 + 边长 + 2*间隔) + 奇偶偏移
                    col as f32 * (HEXAGON_SIZE + HEXAGON_SIDE_LENGTH + HEXAGON_GAP * 2.) + offset,
                    // y = 一半高度+一间隔
                    -(row as f32 * (HEXAGON_GAP + HEXAGON_HALF_SIZE)),
                ),
            coordinate: (col, row),
            terrain: Some(terrain),
            building,
            ..default()
        };
        world_map.map.insert((col, row), tm);
    }
}
