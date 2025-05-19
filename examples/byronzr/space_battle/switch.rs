use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SwitchResource {
    pub detect_test: (Option<Entity>, bool),
    pub enemy_appear: (Option<Entity>, bool),
    pub virtual_turret: (Option<Entity>, bool),
    pub debug_render: Option<Entity>,
    pub weapon_entity: Option<Entity>,
    pub background: Option<Entity>,
}
