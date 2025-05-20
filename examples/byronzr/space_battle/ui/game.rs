use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct UILayoutGame;

#[derive(Debug)]
pub enum GameMenuButton {
    AddSpeed,
    SubSpeed,
    AddTorque,
    SubTorque,
}
