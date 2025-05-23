use crate::components::ship::Hud;
use bevy::prelude::*;

use crate::components::ship::ShipHull;

//
pub fn init_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Hud,
        Mesh2d(meshes.add(Annulus::new(150., 151.))),
        MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
        children![
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 150., 0.),
                children![(
                    Text2d::new("flux: 199"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 125., 0.),
                children![(
                    Text2d::new("capacity: 10"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 100., 0.),
                children![(
                    Text2d::new("cooldown: 5"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 75., 0.),
                children![(
                    Text2d::new("hitpoint: 528"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 50., 0.),
                children![(
                    Text2d::new("speed: 1"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 25., 0.),
                children![(
                    Text2d::new("torque: 1"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(100., 5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
                Transform::from_xyz(180., 0., 0.),
                children![(
                    Text2d::new("braking.dst: 50"),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(-50., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ),
        ],
    ));
}

// hud 只会同步 ship 的中心点,并不需要跟随 ship 的旋转
pub fn sync_hud(
    ship_query: Single<&Transform, (With<ShipHull>, Without<Hud>)>,
    hud_query: Single<&mut Transform, (With<Hud>, Without<ShipHull>)>,
) {
    let pos = ship_query.into_inner();
    let mut hud = hud_query.into_inner();
    hud.translation = pos.translation;
}
