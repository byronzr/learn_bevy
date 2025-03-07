use bevy::{asset::LoadedFolder, prelude::*, sprite::Anchor, utils::HashSet};
use rand::{rng, seq::IndexedRandom};

pub mod env;
pub mod utils;
use env::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_state::<GameState>();
    app.init_resource::<PretreatSet>();
    app.init_resource::<WorldMap>();

    // init player info
    let inf = PlayerInfo {
        coordiate: (7, 14),
        movement_range: 2,
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
    app.add_systems(
        RunFixedMainLoop,
        animate_player.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
    );
    app.add_systems(OnEnter(GameState::GenerateWorld), generate_map_data);
    app.add_systems(OnEnter(GameState::InGame), render_map);
    app.run();
}

// 清理迷雾
fn clear_fow(
    mut query: Query<(&mut Sprite, &FowCoor, &mut FowLevel)>,
    player_info: Res<PlayerInfo>,
    mut world_map: ResMut<WorldMap>,
) {
    // 初始化 fow_range
    if world_map.fow_range.is_none() {
        utils::load_fow(&mut world_map);
    }

    let fow = world_map.fow_range.as_ref().unwrap();

    let mut reachable_set = HashSet::new();
    for (mut sprite, fow_coor, mut fow_level) in query.iter_mut() {
        let current = (
            fow_coor.0 as i32 - player_info.coordiate.0 as i32,
            fow_coor.1 as i32 - player_info.coordiate.1 as i32,
        );
        // 按玩家所在奇偶行取不同的范围
        let Some(range) = fow.range.get(player_info.coordiate.1 % 2) else {
            continue;
        };
        for (level, iter) in range.iter().enumerate() {
            for v in iter {
                if current == *v {
                    if level < fow_level.0 {
                        fow_level.0 = level;
                        let alpha = fow_level.0 as f32 / player_info.sight_range as f32;
                        sprite.color = Color::srgba(0., 0., 0., alpha);
                    }
                    // sprite.color = Color::srgba(0., 0., 0., 0.);
                    if level > 0 && level <= player_info.movement_range {
                        reachable_set.insert((fow_coor.0, fow_coor.1));
                    }
                }
            }
        }
    }
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

    if distance > 1. {
        // 找到目标方向,不需要转换
        let front = to_direction / distance;
        let step = speed * time.delta_secs() * front;
        transform.translation += step.extend(0.);
        if *state != PlayerState::Walk {
            *state = PlayerState::Walk;
            *sprite = pretreat
                .player
                .get(&PlayerState::Walk)
                .unwrap()
                .sprite
                .clone();
            player_info.flipping = if to_direction.x < 0. { true } else { false }
        }
        let Some(atlas) = &mut sprite.texture_atlas else {
            return;
        };
        atlas.index += 1;
        if atlas.index > 9 {
            atlas.index = 0;
        }
    } else {
        if *state != PlayerState::Idle {
            *state = PlayerState::Idle;
            *sprite = pretreat
                .player
                .get(&PlayerState::Idle)
                .unwrap()
                .sprite
                .clone();
            transform.translation = player_info.destination.extend(PLAYER_LAYER);
            if let Some(coor) = world_map.destination_coordiate.take() {
                player_info.coordiate = coor;
            }
        }
        let Some(atlas) = &mut sprite.texture_atlas else {
            return;
        };
        atlas.index += 1;
        if atlas.index > indices.last {
            atlas.index = indices.first;
        }
    }
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
    let Some(event) = events.read().last() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.position) else {
        return;
    };
    for (mut terrain, transform, marker) in &mut query {
        if utils::point_in_flat_top_hexagon(
            world_position,
            transform.translation.truncate(),
            HEXAGON_SIDE_LENGTH,
        ) {
            let coor = (marker.0, marker.1);
            if world_map.reachable_coordiate_set.contains(&coor) {
                terrain.color = Color::srgba(0., 1., 0., 0.5);
                if input.just_pressed(MouseButton::Left) {
                    player_info.destination = transform.translation.truncate();
                    world_map.destination_coordiate = Some(coor);
                }
            } else {
                terrain.color = Color::srgba(1., 0., 0., 0.5);
            }
        } else {
            terrain.color = Color::WHITE;
        }
    }
}

// 初始化地图
fn render_map(
    asset_server: Res<AssetServer>,
    world_map: Res<WorldMap>,
    mut commands: Commands,
    pretreat: Res<PretreatSet>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let mut start_position = Vec2::new(0., 0.);
    let start_coordiante = (7, 14);
    for col in 0..COLS {
        for row in 0..ROWS {
            let Some(tm) = world_map.map.get(&(col, row)) else {
                error!("tile map not found.");
                return;
            };

            // 地形
            let Some(ei) = tm.terrain.clone() else {
                // 地形是必须的
                return;
            };

            if (col, row) == start_coordiante {
                start_position = tm.position;
            }

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
            let mut fow = pretreat.fow.sprite.clone();
            fow.color = Color::srgba(0., 0., 0., 1.);
            commands
                .spawn((
                    fow,
                    FowCoor(col, row),
                    FowLevel(99),
                    Transform::from_translation(tm.position.extend(FOW_LAYER)),
                ))
                .with_children(|parent| {
                    // 坐标
                    parent.spawn((
                        // 差值坐标
                        Text2d(format!(
                            "{},{}",
                            col as i32 - start_coordiante.0 as i32,
                            row as i32 - start_coordiante.1 as i32
                        )),
                        // 原始坐标
                        //Text2d(format!("{},{}", col, row)),
                        TextFont {
                            font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                            font_size: 12.0,
                            ..default()
                        },
                        //TextColor(Color::BLACK),
                        Transform::from_translation(Vec3::new(0., 0., COORDIANTE_LAYER)),
                        //Visibility::Hidden,
                    ));
                });
        }
    }

    // Player
    if let Some(idle) = pretreat.player.get(&PlayerState::Idle) {
        let mut transform = Transform::from_translation(start_position.extend(PLAYER_LAYER));
        transform.scale = Vec3::splat(0.75);
        commands.spawn((
            idle.sprite.clone(),
            AnimationIndices { first: 0, last: 5 },
            PlayerTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            PlayerState::Idle,
            transform,
        ));
        player_info.destination = start_position;
        player_info.coordiate = start_coordiante;
    }
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
            // 归纳所有图片
            // tile 96 x 768 = 3 x 24
            let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 24, None, None);
            let handle_layout = textures.add(atlas_layout);

            // handle map
            let handle = asset_server.load("textures/HexTilesetv3.png");

            // 以下定义,应当来自于数据库
            // ------------- 地形载入 ----------------
            let terrains = [
                "mound",
                "glacier",
                "snowfield",
                "grassland",
                "swamp",
                "lake",
            ];
            for (idx, name) in terrains.iter().enumerate() {
                pretreat.terrain.push(Some(ElementInfo {
                    name: name.to_string(),
                    layer: TERRAIN_LAYER,
                    sprite: Sprite {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: handle_layout.clone(),
                            index: idx,
                        }),
                        ..default()
                    },
                    ..default()
                }));
            }

            // make fow
            pretreat.fow = ElementInfo {
                name: "fow".to_string(),
                layer: FOW_LAYER,
                sprite: Sprite {
                    image: handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle_layout.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                ..default()
            };

            // ------------- 建筑载入 ---------------- offset = 6
            let buildings = ["1", "2", "3", "4", "5", "6"];
            for (idx, name) in buildings.iter().enumerate() {
                pretreat.building.push(Some(ElementInfo {
                    name: name.to_string(),
                    layer: BUILDING_LAYER,
                    sprite: Sprite {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: handle_layout.clone(),
                            index: idx + 6,
                        }),
                        ..default()
                    },
                    ..default()
                }));
            }
            // 加入 None 使随机时,减少每个地形出现建筑的机率
            for _ in 0..5 {
                pretreat.building.push(None);
            }

            // player idle
            let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 6, 1, None, None);
            let handle_player_layout = textures.add(player_layout);
            let handle_player = asset_server.load("textures/Idle.png");
            pretreat.player.insert(
                PlayerState::Idle,
                ElementInfo {
                    name: "Idle".to_string(),
                    layer: PLAYER_LAYER,
                    sprite: Sprite {
                        image: handle_player.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: handle_player_layout.clone(),
                            index: 0,
                        }),
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
            );

            // player walk
            let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 10, 1, None, None);
            let handle_player_layout = textures.add(player_layout);
            let handle_player = asset_server.load("textures/Walk.png");
            pretreat.player.insert(
                PlayerState::Walk,
                ElementInfo {
                    name: "Walk".to_string(),
                    layer: PLAYER_LAYER,
                    sprite: Sprite {
                        image: handle_player.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: handle_player_layout.clone(),
                            index: 0,
                        }),
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    ..default()
                },
            );

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
    for col in 0..COLS {
        for row in 0..ROWS {
            // 随机地形
            let Some(terrain) = pretreat.terrain.choose(&mut rng).cloned() else {
                error!("can't choose terrain.");
                return;
            };

            let Some(building) = pretreat.building.choose(&mut rng).cloned() else {
                return;
            };
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
                        col as f32 * (HEXAGON_SIZE + HEXAGON_SIDE_LENGTH + HEXAGON_GAP * 2.)
                            + offset,
                        // y = 一半高度+一间隔
                        -(row as f32 * (HEXAGON_GAP + HEXAGON_HALF_SIZE)),
                    ),
                coordinate: (col, row),
                terrain,
                building,
                ..default()
            };
            world_map.map.insert((col, row), tm);
        }
    }
}
