use bevy::{prelude::*, window::WindowResolution};

use bevy_rapier2d::prelude::*;

mod components;
mod events;
mod resources;
mod strategies;
mod ui;
mod utility;

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

    app.init_resource::<resources::state::MainMenu>();
    app.init_resource::<resources::state::GameMenu>();

    // 注意: 世界真奇妙, Interaction 响应延时的元凶居然是 magnet.app 或  BetterTouchTool.app
    // https://github.com/bevyengine/bevy/issues/10658
    //app.insert_resource(WinitSettings::desktop_app());

    app.add_plugins(ui::UIPlugin);
    app.add_plugins(strategies::StrategiesPlugin);
    app.add_plugins(resources::ResourcePlugin);

    app.run();
}
