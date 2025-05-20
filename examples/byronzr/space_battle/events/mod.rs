use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct Emit {
    pub direction: Vec2,
    pub start_position: Vec2,
}
