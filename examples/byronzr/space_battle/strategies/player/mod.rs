use crate::components::weapon::WeaponType;
use bevy::prelude::*;
pub mod player_detection;
pub use player_detection::*;

pub mod generate;
pub use generate::*;

pub mod drift;
pub use drift::*;

pub mod hud;
pub use hud::*;

use crate::resources::player::PlayerShipResource;

pub fn load_weapon_sounds(
    mut ship: ResMut<PlayerShipResource>,
    assert_server: ResMut<AssetServer>,
) {
    let beam_sound = assert_server.load("space_battle/audio/disintegrator_fire_01.ogg");
    ship.weapon_sounds
        .insert(WeaponType::Beam, beam_sound.clone());
    ship.weapon_sounds
        .insert(WeaponType::Bullet, beam_sound.clone());
    ship.weapon_sounds
        .insert(WeaponType::Missile, beam_sound.clone());
}
