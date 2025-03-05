use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap, winit::WinitSettings};
use rand::{rng, seq::IndexedRandom};

// 分层
const TERRAIN_LAYER: f32 = 1.; // 地形
const BUILDING_LAYER: f32 = 2.; // 建筑
const NPC_LAYER: f32 = 3.; // NPC
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

// 配合数据库前期完成载入与归类
// 用于随机分配的集合(loading完成分配)
#[derive(Resource, Debug, Default)]
struct PretreatSet {
    pub window_size: Vec2,
    pub terrain: Vec<Option<ElementInfo>>,
    pub building: Vec<Option<ElementInfo>>,
    pub npc: Vec<Option<ElementInfo>>,
    pub player: Vec<Option<ElementInfo>>,
}

// 纹理集中目录
#[derive(Resource, Debug)]
struct LoadTexture(Handle<LoadedFolder>);

// 游戏状态
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Loading, // 加载
    GenerateWorld, // 创建或读取世界
    InGame,        // 游戏进行中
}

// 玩家能力信息
#[derive(Resource, Debug)]
struct PlayerInfo {
    pub movement_range: usize, // 移动距离
    pub sight_range: usize,    // 视野距离
}

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
    app.insert_resource(WinitSettings::desktop_app());
    app.add_systems(OnEnter(GameState::Loading), load_textures);
    app.add_systems(
        Update, // after 确保 LoadTexture 资源被放入
        check_textures.after(load_textures),
    );
    app.add_systems(OnEnter(GameState::GenerateWorld), generate_map_data);
    app.add_systems(OnEnter(GameState::InGame), render_map);
    app.run();
}

fn render_map(world_map: Res<WorldMap>, mut commands: Commands) {
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

            commands.spawn((
                ei.sprite,
                Transform::from_translation(tm.position.extend(TERRAIN_LAYER)),
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
        }
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
            // 加入两个 None 使随机时,减少每个地形出现建筑的机率
            pretreat.building.push(None);
            pretreat.building.push(None);

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
