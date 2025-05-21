use crate::components::ship::ShipState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerShipResource {
    pub ship_entity: Option<Entity>,             // entity
    pub state: ShipState,                        // 飞船状态
    pub sprite: Option<Sprite>,                  // 飞船精灵
    pub mesh2d: Option<Handle<Mesh>>,            // 飞船网格
    pub material: Option<Handle<ColorMaterial>>, // 飞船材质
    pub target_enmey: Option<Entity>,            // 锁定目标敌人
}
