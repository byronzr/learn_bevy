use bevy::prelude::*;
use bevy::sprite::Anchor;
use serde_json;
use std::fs;

use super::{
    BUILDING_LAYER, ElementInfo, FOW_LAYER, FowSet, PLAYER_LAYER, PlayerState, PretreatSet,
    TERRAIN_LAYER, WorldMap,
};

pub fn load_fow(world_map: &mut WorldMap) {
    let fow_path = "./assets/storage/fow.json";
    let Ok(content) = fs::read_to_string(fow_path) else {
        error!("can't read fow.json");
        return;
    };

    let Ok(fow) = serde_json::from_str::<FowSet>(&content) else {
        error!("can't parse fow.json");
        return;
    };
    world_map.fow_range = Some(fow);
}

pub fn write_fow() {
    // let mut fow_set = FowSet::default();
    // for range in fow.range.iter() {
    //     let mut row = vec![];

    //     for iter in range.iter() {
    //         let mut set = HashSet::new();
    //         for v in iter {
    //             set.insert(*v);
    //         }
    //         row.push(set);
    //     }
    //     fow_set.range.push(row);
    // }

    // write to file fow_set.json
    // let fow_set_path = "./assets/storage/fow_set.json";
    // let Ok(content) = serde_json::to_string(&fow_set) else {
    //     error!("can't serialize fow_set.json");
    //     return;
    // };
    // let Ok(_) = fs::write(fow_set_path, content) else {
    //     error!("can't write fow_set.json");
    //     return;
    // };
}

/// 是否选中平顶六边形
pub fn point_in_flat_top_hexagon(point: Vec2, hex_center: Vec2, hex_side_length: f32) -> bool {
    let q2x = f32::abs(point.x - hex_center.x);
    let q2y = f32::abs(point.y - hex_center.y);
    let h = hex_side_length * 0.866;
    if q2x > hex_side_length || q2y > h {
        return false;
    }
    if q2x <= hex_side_length * 0.5 {
        return true;
    }
    let q3x = h - (2. * h / hex_side_length) * (q2x - hex_side_length / 2.);
    return q2y <= q3x;
}

pub fn init_pretreat(
    textures: &mut Assets<TextureAtlasLayout>,
    asset_server: &AssetServer,
    pretreat: &mut PretreatSet,
) {
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
}

// debug for fow_range
// if input.just_pressed(MouseButton::Left) {
//     let difference_value = (
//         coor.0 as i32 - player_info.coordiate.0 as i32,
//         coor.1 as i32 - player_info.coordiate.1 as i32,
//     );
//     print!("[{},{}],", difference_value.0, difference_value.1);
//     io::stdout().flush().unwrap();
// }
