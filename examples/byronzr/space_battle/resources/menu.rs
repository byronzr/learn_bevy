use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenu {
    pub enemy_appear: bool,
    pub debug_render: bool,
    pub mesh_mode: bool,
    pub virtual_turret: bool,
    pub detect_test: bool,
    pub log: bool,
    pub engine_flame: bool,
    pub lock_player: bool,
    pub curve: bool,
    pub game_speed: f32,

    // audio
    pub ui_button_pressed: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct GameMenu {
    pub weapon_type: WeaponType,

    pub ui_button_pressed: Handle<AudioSource>,
}
