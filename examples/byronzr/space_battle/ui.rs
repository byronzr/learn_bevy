use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;
use bevy_rapier2d::render::DebugRenderContext;

use crate::{switch::SwitchResource, weapon::WeaponResource};
// use bevy_rapier2d::prelude::*;

// UI 提示
#[derive(Component)]
pub struct Tip;

// UI 检查器
#[derive(Component)]
pub struct Inf;

// 虚拟炮台
#[derive(Component)]
pub struct VirtualTurret(pub bool);

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, show_grid));
        //app.add_systems(Update);
    }
}

fn setup(
    mut commands: Commands,
    mut gizmos_assets: ResMut<Assets<GizmoAsset>>,
    rapier_context: Res<DebugRenderContext>,
    switch: Res<SwitchResource>,
    weapon: Res<WeaponResource>,
) {
    //
    commands.spawn(Camera2d);

    // Tips
    commands.spawn((
        Text(format!(
            "Pause Game: Space
            Switch Debug Render: Tab [{}]
            Enemy Start: S [{}]
            Detect Test: I [{}]
            Virtual Turret: Q 
            Weapon Type: [{:?}]",
            rapier_context.enabled, switch.enemy_start, switch.detect_test, weapon.fire_type
        )),
        TextFont {
            font_size: 12.,
            ..default()
        },
        Tip,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    // Infomation
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: 12.,
            ..default()
        },
        Inf,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            right: Val::Px(12.),
            ..default()
        },
    ));

    let mut gizmos = GizmoAsset::default();
    gizmos.arrow_2d(Vec2::ZERO, Vec2::new(0., 200.), Color::srgb_u8(255, 255, 0));
    commands.spawn((
        Gizmo {
            handle: gizmos_assets.add(gizmos),
            ..default()
        },
        VirtualTurret(false),
        Disabled,
    ));
}

// 显示网格方便观察
fn show_grid(
    mut commands: Commands,
    mut gizom_assets: ResMut<Assets<GizmoAsset>>,
    mut res: ResMut<SwitchResource>,
) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    // 1920 x 1080
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,                   // 投影模式
            UVec2::new(96, 54),                     // 单元格数量
            Vec2::new(20., 20.),                    // 单元格大小
            LinearRgba::gray(0.05).with_alpha(0.2), // 网格颜色
        )
        .outer_edges();
    let id = commands
        .spawn((Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },))
        .id();
    res.background = Some(id);
}
