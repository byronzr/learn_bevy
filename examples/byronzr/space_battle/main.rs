use bevy::{prelude::*, window::WindowResolution};

use bevy_rapier2d::prelude::*;

mod control;
mod debug;
mod enemy;
mod player;
mod switch;
mod ui;
mod weapon;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1920., 1080.),
                    ..default()
                }),
                ..default()
            }),
    );
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.init_resource::<switch::SwitchResource>();

    app.add_plugins(ui::UIPlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(control::ControlsPlugin);
    app.add_plugins(enemy::EnemyPlugin);
    app.add_plugins(weapon::WeaponPlugin);

    app.run();
}
