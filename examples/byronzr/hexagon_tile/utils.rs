use bevy::prelude::*;
use serde_json;
use std::fs;

use super::{FowSet, WorldMap};

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
