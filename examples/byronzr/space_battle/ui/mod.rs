use std::borrow::Cow;

use bevy::{
    core_pipeline::{
        bloom::{Bloom, BloomCompositeMode},
        tonemapping::{DebandDither, Tonemapping},
    },
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    time,
};

use bevy_ecs::entity_disabling::Disabled;
use detect::DebugRenderMaker;

use crate::{components::ship::ShipHull, resources::menu::MainMenu};
pub mod detect;
pub mod game;
pub mod panel;
pub mod statistic;

// UI 提示
#[derive(Component, Eq, PartialEq, Debug)]
pub enum ButtonStatus {
    Active,
    Inactive,
}

// 可能会用的 ui 全局定位资源
#[derive(Resource, Debug, Default)]
pub struct UIResource {
    pub panel: Option<Entity>,
    pub game: Option<Entity>,
    pub detect: Option<Entity>,
    pub statistic: Option<Entity>,
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIResource>();
        app.add_systems(
            Startup,
            (setup, panel::ui_main_setup, game::ui_game_setup, show_grid).chain(),
        );
        app.add_systems(
            Update,
            (
                detect::direct_test,
                panel::button_interaction,
                game::button_interaction,
                zoom,
                lock_player,
                statistic::statistic,
            ),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut gizom_assets: ResMut<Assets<GizmoAsset>>,
    mut ui_resource: ResMut<UIResource>,
) {
    let mut gizmos = GizmoAsset::default();
    // camera
    let mut bloom = Bloom::default();
    bloom.intensity = 0.15;
    bloom.prefilter.threshold = 0.2;
    bloom.low_frequency_boost = 1.0;
    bloom.composite_mode = BloomCompositeMode::Additive;
    commands.spawn((
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Camera2d,
        Tonemapping::TonyMcMapface,
        bloom,
        DebandDither::Enabled,
    ));

    // spawn background with sprite texture
    commands.spawn((
        Sprite {
            // ! 不敢相信,JPEG 不是默认开启的 feature
            image: asset_server.load("space_battle/background4.jpg"),
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0., -100.)),
        // ! Bevy 居然有一个默认组件是 Name 我居然现在
        Name::new("Background"),
    ));

    // UI layout 下面
    let id = commands
        .spawn((
            game::UILayoutGame,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.),
                left: Val::Px(12.),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.),
                ..default()
            },
        ))
        .id();
    ui_resource.game = Some(id);

    // UI layout 左上方
    let id = commands
        .spawn((
            panel::UILayoutMain,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                left: Val::Px(12.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.),
                ..default()
            },
        ))
        .id();
    ui_resource.panel = Some(id);

    // UI layout debug 右上方
    let id = commands
        .spawn((
            Text::default(),
            TextFont {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 12.0,
                ..default()
            },
            detect::UILayoutDetect,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                right: Val::Px(12.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.),
                ..default()
            },
        ))
        .id();
    ui_resource.detect = Some(id);

    // UI layout statistic 右下方
    let id = commands
        .spawn((
            Text::default(),
            TextFont {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 12.0,
                ..default()
            },
            statistic::UILayoutStatistic,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.),
                right: Val::Px(12.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.),
                ..default()
            },
        ))
        .id();
    ui_resource.statistic = Some(id);

    // virtual turret
    // (虚拟炮塔方向指针)红色
    gizmos.arrow_2d(Vec2::ZERO, Vec2::new(0., 200.), Color::srgb_u8(255, 0, 0));
    commands.spawn((
        detect::VirtualTurret,
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
        Disabled,
    ));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * 0.05;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(0.1, 10.);
        }
        Projection::Perspective(ref mut perspective) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * 0.05;

            // Adjust the field of view, but keep it within our stated range.
            perspective.fov = (perspective.fov + delta_zoom).clamp(0.1, 10.);
        }
        _ => (),
    }
}

fn lock_player(
    mut camera: Single<&mut Transform, (Without<ShipHull>, With<Camera2d>)>,
    player: Single<&Transform, (Without<Camera2d>, With<ShipHull>)>,
    menu: Res<MainMenu>,
    time: Res<Time>,
) {
    // 需要锁定玩家
    if menu.lock_player {
        let Vec3 { x, y, .. } = player.translation;
        let direction = Vec3::new(x, y, camera.translation.z);
        // 平滑轻微调整
        camera
            .translation
            .smooth_nudge(&direction, 2.0, time.delta_secs());
    }
}

// 显示网格方便观察
fn show_grid(mut commands: Commands, mut gizom_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::default();
    // 网格 (1280x720)
    // 1920 x 1080
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,                   // 投影模式
            UVec2::new(96, 54),                     // 单元格数量
            Vec2::new(20., 20.),                    // 单元格大小
            LinearRgba::gray(0.05).with_alpha(0.1), // 网格颜色
        )
        .outer_edges();
    commands.spawn((
        DebugRenderMaker,
        Gizmo {
            handle: gizom_assets.add(gizmos),
            ..default()
        },
    ));
}

fn button<T: Component>(
    asset_server: &mut AssetServer,
    name: Cow<'_, str>,
    id: T,
    active: bool,
) -> impl Bundle {
    (
        if active {
            ButtonStatus::Active
        } else {
            ButtonStatus::Inactive
        },
        if active {
            BackgroundColor(Color::srgb_u8(0, 84, 0))
        } else {
            BackgroundColor(Color::BLACK)
        },
        Button,
        id,
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
        BorderRadius::all(Val::Px(5.0)),
        BorderColor(Color::WHITE.with_alpha(0.2)),
        children![(
            Text::new(name),
            TextFont {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            //TextShadow::default(),
        )],
    )
}
