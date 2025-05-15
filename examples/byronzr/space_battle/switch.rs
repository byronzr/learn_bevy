use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SwitchResource {
    pub detect_test: bool,
    pub engine_start: bool,
    pub background: Option<Entity>,
}
