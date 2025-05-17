use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;
use bevy_rapier2d::prelude::*;
//use rand::{Rng, rng};

use crate::player::ShipHull;

use crate::switch::SwitchResource;
use crate::ui::{Tip, VirtualTurret};
use crate::weapon::{WeaponResource, WeaponType};

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controls);
    }
}

fn controls(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut render_context: ResMut<DebugRenderContext>,
    _player: Single<Entity, With<ShipHull>>,
    virtual_turret: Single<(Entity, &mut VirtualTurret, Option<&Disabled>)>,
    mut switch: ResMut<SwitchResource>,
    mut weapon: ResMut<WeaponResource>,
    mut text: Single<&mut Text, With<Tip>>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        weapon.fire_type = WeaponType::Hamer;
    }
    if keyboard_input.just_pressed(KeyCode::Numpad2) {
        weapon.fire_type = WeaponType::Bullet;
    }
    if keyboard_input.just_pressed(KeyCode::Numpad3) {
        weapon.fire_type = WeaponType::Missile;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        // todo
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        switch.enemy_start = !switch.enemy_start;
    }
    if keyboard_input.just_pressed(KeyCode::Tab) {
        render_context.enabled = !render_context.enabled;
        let Some(entity) = switch.background else {
            return;
        };
        if render_context.enabled {
            commands.entity(entity).remove::<Disabled>();
        } else {
            commands.entity(entity).insert(Disabled);
        }
    }

    // show infomation
    let (entity, mut virtual_turret, _) = virtual_turret.into_inner();
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        switch.detect_test = !switch.detect_test;
        if switch.detect_test {
            commands.entity(entity).remove::<Disabled>();
        } else {
            commands.entity(entity).insert(Disabled);
        }
    }
    // Virtual turret rotate
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        virtual_turret.0 = !virtual_turret.0;
    }
    text.0 = format!(
        "
        Pause Game: Space
        Switch Debug Render: Tab [{}]
        Enemy Start: S [{}]
        Detect Test: I [{}]
        Virtual Turret: Q [{}]
        Weapon Type: [{:?}]",
        render_context.enabled,
        switch.enemy_start,
        switch.detect_test,
        virtual_turret.0,
        weapon.fire_type
    );
}
