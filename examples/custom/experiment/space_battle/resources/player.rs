use crate::components::{ship::ShipState, weapon::WeaponType};
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default)]
pub struct PlayerShipResource {
    pub ship_entity: Option<Entity>,  // entity
    pub state: ShipState,             // 飞船状态
    pub sprite: Option<Sprite>,       // 飞船精灵
    pub target_enmey: Option<Entity>, // 锁定目标敌人
    pub engine_flame: bool,           // 引擎尾焰
    pub pre_weapon_range: f32,        // 上次武器射程(为了减少更新)
    pub weapon_range: f32,            // 武器射程
    // sounds of weapon
    pub weapon_sounds: HashMap<WeaponType, Handle<AudioSource>>,

    // base
    pub speed: f32,
    pub torque: f32,
    pub bk_speed: f32,
    pub bk_torque: f32,
    pub pre_bk_distance: f32,
    pub bk_distance: f32,
}
