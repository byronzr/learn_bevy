use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod control;
mod enemy;
mod player;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(control::ControlsPlugin);
    app.add_plugins(enemy::EnemyPlugin);

    app.add_systems(Startup, (setup, show_grid));

    app.run();
}

fn setup(mut commands: Commands) {
    //
    commands.spawn(Camera2d);

    commands.spawn((
        Text(format!(
            "Pause Game: Space\nSwitch Debug Render: Tab\nThruster: S\nGenerate Enemy: E"
        )),
        Node {
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,                   // 投影模式
            UVec2::new(64, 36),                     // 单元格数量
            Vec2::new(20., 20.),                    // 单元格大小
            LinearRgba::gray(0.05).with_alpha(0.2), // 网格颜色
        )
        .outer_edges();
    commands.spawn((
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Transform::from_xyz(0., 0., -99.),
    ));
}
