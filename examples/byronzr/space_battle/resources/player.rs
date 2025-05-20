use crate::components::ship::ShipState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerShipResource {
    pub ship_entity: Option<Entity>,
    pub state: ShipState,
    pub sprite: Option<Sprite>,
    pub mesh2d: Option<Handle<Mesh>>,
    pub material: Option<Handle<ColorMaterial>>,
}
