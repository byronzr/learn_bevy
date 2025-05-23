use bevy::prelude::*;

use crate::components::weapon::WeaponType;

#[derive(Event, Debug)]
pub struct Emit {
    pub direction: Vec2,
    pub start_position: Vec2,
    pub weapon_type: WeaponType,
}

#[derive(Event, Debug)]
pub struct SeekEnemy {
    pub enemy_entity: Entity,
}

#[derive(Event, Debug)]
pub struct EventFlame {
    pub start: bool,
}
