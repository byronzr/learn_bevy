use crate::{
    resources::{menu::MainMenu, player::PlayerShipResource},
    shader::MaterialEngineFlame,
};
use bevy::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_time);
    }
}

fn update_time(
    mut custom_materials: ResMut<Assets<MaterialEngineFlame>>,
    ship: Res<PlayerShipResource>,
    menu: Res<MainMenu>,
) {
    for material in custom_materials.iter_mut() {
        if ship.engine_flame {
            material.1.start();
        } else {
            material.1.stop();
        }
    }
    if menu.log {
        println!("engine flame: {:?}", ship.engine_flame);
    }
}
