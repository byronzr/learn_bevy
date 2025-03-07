use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

// 分层
pub const TERRAIN_LAYER: f32 = 1.; // 地形
pub const BUILDING_LAYER: f32 = 2.; // 建筑
pub const NPC_LAYER: f32 = 3.; // NPC
pub const FOW_LAYER: f32 = 5.; // 雾
pub const COORDIANTE_LAYER: f32 = 6.; // 坐标
pub const PLAYER_LAYER: f32 = 99.; // 玩家

// 地图块个数
pub const COLS: usize = 16; // 列
pub const ROWS: usize = 32; // 行

// 地图可能需要偏移
pub const MAP_OFFSET: Vec2 = Vec2 { x: 16., y: 16. };

// Hexagon
pub const HEXAGON_SIZE: f32 = 32.; // 宽高
pub const HEXAGON_HALF_SIZE: f32 = 16.; // 一半
pub const HEXAGON_GAP: f32 = 1.; // 间隔
pub const HEXAGON_SIDE_LENGTH: f32 = 18.; // 边长
pub const HEXAGON_SIDE_WIDTH: f32 = 7.; // 侧边宽度

// 迷雾逐圈数据
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FowRange {
    // 奇偶行不一样
    // [奇偶行][等级][坐标]
    pub range: Vec<Vec<Vec<(i32, i32)>>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FowSet {
    // 奇偶行不一样
    // [奇偶行][等级][坐标]
    pub range: Vec<Vec<HashSet<(i32, i32)>>>,
}

// 玩家状态
#[derive(Debug, Hash, PartialEq, Eq, Clone, Component)]
pub enum PlayerState {
    Idle,
    Walk,
    Run,
    Attack,
    Die,
}

// 配合数据库前期完成载入与归类
// 用于随机分配的集合(loading完成分配)
#[derive(Resource, Debug, Default)]
pub struct PretreatSet {
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
pub struct LoadTexture(pub Handle<LoadedFolder>);

// 动画指示器(通用)
// 起始桢都是 0 所以简化了
#[derive(Component, Debug, Default)]
pub struct AnimationIndices {
    pub idle: usize,
    pub walk: usize,
}

// 玩家动画频率
#[derive(Component, Debug, Default)]
pub struct PlayerTimer(pub Timer);

// 游戏状态
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Loading, // 加载
    GenerateWorld, // 创建或读取世界
    InGame,        // 游戏进行中
}

// 玩家能力信息
#[derive(Resource, Debug, Default)]
pub struct PlayerInfo {
    pub coordiate: (usize, usize), // 当前坐标
    pub movement_range: usize,     // 移动距离
    pub sight_range: usize,        // 视野距离
    pub destination: Vec2,         // 目标
    pub flipping: bool,            // 翻转(记录着最后一次移动方向)
}

// FOW 坐标
#[derive(Debug, Component)]
pub struct FowCoor(pub usize, pub usize);

// FOW 等级
#[derive(Debug, Component)]
pub struct FowLevel(pub usize);

// 地形标记
#[derive(Debug, Default, Component)]
pub struct TerrainMarker(pub usize, pub usize);

// 可视元素基本信息(AtlasInfo)
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementInfo {
    pub name: String,
    pub path: String,
    pub layer: f32,
    #[serde(skip)]
    pub sprite: Sprite,
    pub description: String,
}

// 每个 Tile 的具体信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TileMap {
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
pub struct WorldMap {
    pub map: HashMap<(usize, usize), TileMap>,
    pub destination_coordiate: Option<(usize, usize)>, // 玩家移动目标,缓存至移动完毕
    pub reachable_coordiate_set: HashSet<(usize, usize)>, // 可达坐标
    pub fow_range: Option<FowSet>,                     // 雾逐圈数据
    pub coordiate_combined: Vec<(usize, usize)>,       // 笛卡尔积(Cartesian Product最大组合)
}

impl WorldMap {
    pub fn init_coordiate_combined(&mut self) -> Vec<(usize, usize)> {
        if self.coordiate_combined.len() > 0 {
            return self.coordiate_combined.clone();
        }
        for x in 0..COLS {
            for y in 0..ROWS {
                self.coordiate_combined.push((x, y));
            }
        }
        return self.coordiate_combined.clone();
    }
}
