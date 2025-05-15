use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::{VirtualTurret, player::ShipHull};

//use rand_chacha::ThreadRng;

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
    player: Single<Entity, With<ShipHull>>,
    virtual_turret: Single<(Entity, &mut VirtualTurret, Option<&Disabled>)>,
    mut switch: ResMut<crate::SwitchResource>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // todo
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        let mut rng = rng();
        let (x, y) = (
            rng.random_range(-1000.0..1000.0),
            rng.random_range(-1000.0..1000.0),
        );
        commands.entity(*player).insert(ExternalImpulse {
            impulse: Vec2::new(x, y),
            // 正数逆时针
            torque_impulse: -10000.0,
        });
    }
    if keyboard_input.just_pressed(KeyCode::Tab) {
        render_context.enabled = !render_context.enabled;
    }
    let (entity, mut virtual_turret, _) = virtual_turret.into_inner();
    // show infomation
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        switch.detect_test = !switch.detect_test;
        if switch.detect_test {
            commands.entity(entity).remove::<Disabled>();
        } else {
            commands.entity(entity).insert(Disabled);
        }
        println!("detect_test: {}", switch.detect_test);
    }
    // Virtual turret rotate
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        virtual_turret.0 = !virtual_turret.0;
    }
    // todo
}
