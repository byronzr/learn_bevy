use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rng};

use crate::player::ShipHull;

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
    // generate a random enemy
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        // todo
    }
}
