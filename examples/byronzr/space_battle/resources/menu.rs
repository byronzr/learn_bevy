use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenu {
    pub enemy_appear: bool,
    pub debug_render: bool,
    pub mesh_mode: bool,
    pub virtual_turret: bool,
    pub detect_test: bool,
    pub weapon_type: WeaponType,
}

#[derive(Resource, Default)]
pub struct GameMenu {
    //todo
}
