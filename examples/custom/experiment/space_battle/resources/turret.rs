use crate::components::weapon::{Weapon, WeaponType};
use bevy::prelude::*;

#[derive(Resource)]
pub struct TurretResource {
    pub weapon: Vec<Weapon>,
    pub fire_type: WeaponType,
}

impl TurretResource {
    pub fn available_weapons(&mut self) -> Vec<&mut Weapon> {
        self.weapon
            .iter_mut()
            .filter(|w| w.weapon_type == self.fire_type)
            .collect::<Vec<_>>()
    }

    pub fn current_range(&mut self) -> f32 {
        for weapon in self.available_weapons() {
            if let Some(phase) = weapon.phase.first() {
                return phase.range;
            }
        }
        0.0
    }
}
