use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct EngineFlame;

#[derive(Debug, Component)]
pub struct SeekFlag(pub Timer);
