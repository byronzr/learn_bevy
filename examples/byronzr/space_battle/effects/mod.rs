use crate::{
    resources::{menu::MainMenu, player::PlayerShipResource},
    shader::MaterialEngineFlame,
};
use bevy::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, engine_flame);
    }
}

fn engine_flame(
    mut custom_materials: ResMut<Assets<MaterialEngineFlame>>,
    ship: Res<PlayerShipResource>,
    menu: Res<MainMenu>,
) {
    if !menu.engine_flame {
        return;
    }
    for material in custom_materials.iter_mut() {
        if ship.engine_flame {
            material.1.start();
        } else {
            material.1.stop();
        }
    }
}
