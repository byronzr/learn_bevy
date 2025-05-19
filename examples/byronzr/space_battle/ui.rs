use std::borrow::Cow;

use bevy::prelude::*;
use bevy_ecs::entity_disabling::Disabled;

use crate::{switch::SwitchResource, turret::WeaponResource};
// use bevy_rapier2d::prelude::*;

// UI 提示
#[derive(Component)]
pub struct ActiveButton;

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
    mut switch: ResMut<SwitchResource>,
    weapon: Res<WeaponResource>,
    mut asset_server: ResMut<AssetServer>,
) {
    //
    commands.spawn(Camera2d);

    // Tips
    let tips = commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.),
            left: Val::Px(12.),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(12.),
            ..default()
        },))
        .id();
    switch.enemy_appear = (
        Some(
            commands
                .spawn(button(&mut asset_server, "Enemy Appear".into()))
                .insert(ChildOf(tips))
                .id(),
        ),
        false,
    );
    switch.debug_render = Some(
        commands
            .spawn(button(&mut asset_server, "Debug Render".into()))
            .insert(ChildOf(tips))
            .id(),
    );

    switch.detect_test = (
        Some(
            commands
                .spawn(button(&mut asset_server, "Detect Test".into()))
                .insert(ChildOf(tips))
                .id(),
        ),
        false,
    );

    switch.virtual_turret = (
        Some(
            commands
                .spawn(button(&mut asset_server, "Virtual Turret".into()))
                .insert(ChildOf(tips))
                .id(),
        ),
        false,
    );

    switch.weapon_entity = Some(
        commands
            .spawn(button(
                &mut asset_server,
                format!("Weapon Type: {:?}", weapon.fire_type).into(),
            ))
            .insert(ChildOf(tips))
            .id(),
    );

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

fn button(asset_server: &mut AssetServer, name: Cow<'_, str>) -> impl Bundle + use<> {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(25.0),
            border: UiRect::all(Val::Px(1.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        //BorderColor(Color::BLACK),
        BackgroundColor(Color::BLACK),
        BorderRadius::all(Val::Px(5.0)),
        children![(
            Text::new(name),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}
