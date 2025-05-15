use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

mod control;
mod debug;
mod enemy;
mod player;
mod switch;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.init_resource::<switch::SwitchResource>();

    app.add_plugins(ui::UIPlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(control::ControlsPlugin);
    app.add_plugins(enemy::EnemyPlugin);

    app.run();
}
