use std::time::Duration;

use bevy::{
    asset::LoadedFolder,
    ecs::event,
    prelude::*,
    sprite::Anchor,
    utils::HashMap,
    winit::{UpdateMode, WinitSettings},
};
use rand::{rng, seq::IndexedRandom};

// 分层
const TERRAIN_LAYER: f32 = 1.; // 地形
const BUILDING_LAYER: f32 = 2.; // 建筑
const NPC_LAYER: f32 = 3.; // NPC
const FOW_LAYER: f32 = 5.; // 雾
const COORDIANTE_LAYER: f32 = 6.; // 坐标
const PLAYER_LAYER: f32 = 99.; // 玩家

// 地图块个数
const COLS: usize = 16; // 列
const ROWS: usize = 32; // 行

// 地图可能需要偏移
const MAP_OFFSET: Vec2 = Vec2 { x: 16., y: 16. };

// Hexagon
const HEXAGON_SIZE: f32 = 32.; // 宽高
const HEXAGON_HALF_SIZE: f32 = 16.; // 一半
const HEXAGON_GAP: f32 = 1.; // 间隔
const HEXAGON_SIDE_LENGTH: f32 = 18.; // 边长
const HEXAGON_SIDE_WIDTH: f32 = 7.; // 侧边宽度

// 可视元素基本信息(AtlasInfo)
#[derive(Component, Debug, Clone, Default)]
struct ElementInfo {
    pub name: String,
    pub layer: f32,
    pub sprite: Sprite,
    pub description: String,
}

// 玩家状态
#[derive(Debug, Hash, PartialEq, Eq, Clone, Component)]
enum PlayerState {
    Idle,
    Walk,
    Run,
    Attack,
    Die,
}

// 配合数据库前期完成载入与归类
// 用于随机分配的集合(loading完成分配)
#[derive(Resource, Debug, Default)]
struct PretreatSet {
    pub window_size: Vec2,                         // 窗口大小
    pub terrain: Vec<Option<ElementInfo>>,         // 地形
    pub building: Vec<Option<ElementInfo>>,        // 建筑
    pub npc: Vec<Option<ElementInfo>>,             // NPC
    pub player: HashMap<PlayerState, ElementInfo>, // 玩家
    pub fow: ElementInfo,                          // 雾
    pub fow_level: Vec<Vec<(i32, i32)>>,           // 雾等级
}

// 纹理集中目录
#[derive(Resource, Debug)]
struct LoadTexture(Handle<LoadedFolder>);

// 动画指示器(通用)
#[derive(Component, Debug, Default)]
struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

// 玩家动画频率
#[derive(Component, Debug, Default)]
struct PlayerTimer(Timer);

// 游戏状态
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Loading, // 加载
    GenerateWorld, // 创建或读取世界
    InGame,        // 游戏进行中
}

// 玩家能力信息
#[derive(Resource, Debug, Default)]
struct PlayerInfo {
    pub coordiate: (usize, usize), // 当前坐标
    pub movement_range: usize,     // 移动距离
    pub sight_range: usize,        // 视野距离
    pub destination: Vec2,         // 目标
}

// FOW 等级
#[derive(Debug, Component)]
struct FowCoor(usize, usize);

#[derive(Debug, Default, Component)]
struct TerrainMarker;

#[derive(Debug, Default, Component)]
struct CoorRange(usize);

// 每个 Tile 的具体信息
#[derive(Debug, Default)]
struct TileMap {
    pub position: Vec2,
    pub coordinate: (usize, usize),
    pub terrain: Option<ElementInfo>,
    pub building: Option<ElementInfo>,
    pub npc: Option<ElementInfo>,
    pub player: Option<ElementInfo>,
    pub description: Option<String>,
}
// 世界大地图(每个tile)
#[derive(Resource, Debug, Default)]
struct WorldMap {
    pub map: HashMap<(usize, usize), TileMap>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_state::<GameState>();
    app.init_resource::<PretreatSet>();
    app.init_resource::<WorldMap>();
    app.insert_resource(WinitSettings {
        focused_mode: UpdateMode::reactive_low_power(Duration::from_millis(100)),
        ..default()
    });
    // init player info
    let inf = PlayerInfo {
        coordiate: (7, 14),
        movement_range: 1,
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
            mouse_over,
        ),
    );
    app.add_systems(FixedPostUpdate, animate_player.after(render_map));
    app.add_systems(OnEnter(GameState::GenerateWorld), generate_map_data);
    app.add_systems(OnEnter(GameState::InGame), render_map);
    app.run();
}

fn clear_fow(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &FowCoor)>,
    player_info: Res<PlayerInfo>,
) {
    let mut range = vec![];
    range.push(vec![(0, 0)]);
    range.push(
        // level 1
        vec![(0, 0), (0, -2), (0, -1), (0, 1), (0, 2), (-1, 1), (-1, -1)],
    );
    range.push(
        // level 2
        vec![
            (0, -4),
            (0, -3),
            (1, -2),
            (1, 0),
            (1, 2),
            (0, 3),
            (0, 4),
            (-1, 3),
            (-1, 2),
            (-1, 0),
            (-1, -2),
            (-1, -3),
        ],
    );

    range.push(
        // level 3
        vec![
            (0, -6),
            (0, -5),
            (1, -4),
            (1, -3),
            (1, -1),
            (1, 1),
            (1, 3),
            (1, 4),
            (0, 5),
            (0, 6),
            (-1, 5),
            (-1, 4),
            (-2, 3),
            (-2, 1),
            (-2, -1),
            (-2, -3),
            (-1, -4),
            (-1, -5),
        ],
    );

    for (entity, mut sprite, coor) in query.iter_mut() {
        let current = (
            coor.0 as i32 - player_info.coordiate.0 as i32,
            coor.1 as i32 - player_info.coordiate.1 as i32,
        );
        for (level, iter) in range.iter().enumerate() {
            for v in iter {
                if current == *v {
                    let alpha = level as f32 / player_info.sight_range as f32;
                    if alpha < 0. {
                        commands.entity(entity).despawn();
                    } else {
                        sprite.color = Color::srgba(0., 0., 0., alpha);
                    }
                    Color::srgba(0., 0., 0., level as f32 / player_info.sight_range as f32);
                } else {
                }
            }
        }
    }
}

// animate player
fn animate_player(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerTimer,
        &mut Sprite,
        &AnimationIndices,
        &mut Transform,
        &mut PlayerState,
    )>,
    player_info: Res<PlayerInfo>,
    pretreat: Res<PretreatSet>,
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
        // TODO: 镜像
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
            if to_direction.x < 0. {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
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
        }
        let Some(atlas) = &mut sprite.texture_atlas else {
            return;
        };
        atlas.index += 1;
        if atlas.index > indices.last {
            atlas.index = indices.first;
        }
    }
}

/// 是否选中平顶六边形
/// hex_radius 边长,不是半径
pub fn point_in_flat_top_hexagon(point: Vec2, hex_center: Vec2, hex_radius: f32) -> bool {
    let q2x = f32::abs(point.x - hex_center.x);
    let q2y = f32::abs(point.y - hex_center.y);
    let h = hex_radius * 0.866;

    if q2x > hex_radius || q2y > h {
        return false;
    }
    if q2x <= hex_radius * 0.5 {
        return true;
    }

    let q3x = h - (2. * h / hex_radius) * (q2x - hex_radius / 2.);
    return q2y <= q3x;
}

fn mouse_over(
    mut events: EventReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Sprite, &Transform), With<TerrainMarker>>,
    input: Res<ButtonInput<MouseButton>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let Some(event) = events.read().last() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.position) else {
        return;
    };

    for (mut terrain, transform) in &mut query {
        if point_in_flat_top_hexagon(
            world_position,
            transform.translation.truncate(),
            HEXAGON_SIDE_LENGTH,
        ) {
            terrain.color = Color::srgba(0., 1., 0., 0.5);
            if input.just_pressed(MouseButton::Left) {
                player_info.destination = transform.translation.truncate();
            }
        } else {
            terrain.color = Color::WHITE;
        }
    }
}

// 初始化地图
fn render_map(
    world_map: Res<WorldMap>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                TerrainMarker,
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
                    Transform::from_translation(tm.position.extend(FOW_LAYER)),
                ))
                .with_children(|parent| {
                    // 坐标
                    // parent.spawn((
                    //     Text2d(format!(
                    //         "{},{}",
                    //         col as i32 - start_coordiante.0 as i32,
                    //         row as i32 - start_coordiante.1 as i32
                    //     )),
                    //     TextFont {
                    //         font: asset_server.load("fonts/SourceHanSansCN-Normal.otf"),
                    //         font_size: 12.0,
                    //         ..default()
                    //     },
                    //     //TextColor(Color::BLACK),
                    //     Transform::from_translation(Vec3::new(0., 0., COORDIANTE_LAYER)),
                    // ));
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
            PlayerTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            PlayerState::Idle,
            transform,
        ));
        player_info.destination = start_position;
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
        info!("checking...");
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

fn random_data(world_map: &mut WorldMap, pretreat: &PretreatSet) {
    let mut rng = rng();
    let center = pretreat.window_size / 2.;
    // 起始左上角 = 在上角坐标 + (地块一半+整体偏移量)
    let topleft = Vec2::new(-center.x, center.y)
        + (Vec2::splat(HEXAGON_HALF_SIZE) + MAP_OFFSET) * Vec2::new(1., -1.);
    println!("center: {}", center);
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
