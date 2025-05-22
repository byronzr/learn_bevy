use bevy::prelude::*;

pub mod setup;
pub use setup::*;

pub mod interaction;
pub use interaction::*;

#[derive(Component, Debug)]
pub struct UILayoutGame;

#[derive(Debug, Component)]
pub enum GameMenuButton {
    WeaponType,
    // speed
    AddSpeed,
    SubSpeed,
    // torque
    AddTorque,
    SubTorque,

    // braking
    AddBrakingDistance,
    SubBrakingDistance,
    AddBrakingSpeed,
    SubBrakingSpeed,
    AddBrakingTorque,
    SubBrakingTorque,
}
