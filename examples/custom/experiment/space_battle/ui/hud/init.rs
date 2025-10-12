use crate::components::ship::{Hud, HudText};
use bevy::prelude::*;

//
pub fn init_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let hud = commands
        .spawn((
            Hud,
            Mesh2d(meshes.add(Annulus::new(60., 61.))),
            MeshMaterial2d(materials.add(Color::srgba(0., 0.3, 0., 0.1))),
        ))
        .id();

    let status_bar = vec![
        (HudText::Flux, "flux: 199"),
        (HudText::Capacity, "capacity: 10"),
        (HudText::Cooldown, "cooldown: 5"),
        (HudText::Hitpoint, "hitpoint: 528"),
        (HudText::Speed, "speed: 1"),
        (HudText::Torque, "torque: 1"),
        (HudText::BkDistance, "bk.dst: 50"),
        (HudText::BkSpeed, "bk.speed: 1"),
        (HudText::BkTorque, "bk.torque: 1"),
    ];
    let start_y = 150.;
    for (idx, (hudtext, text_str)) in status_bar.iter().enumerate() {
        commands
            .spawn((
                *hudtext,
                Sprite {
                    rect: Some(Rect::new(0., 0., 100., 5.)),
                    color: Color::srgba(0., 0.5, 0., 0.2),
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                Transform::from_xyz(90., start_y - (25. * idx as f32), 0.),
                children![(
                    Text2d::new(*text_str),
                    TextFont {
                        font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
                        font_size: 10.,
                        ..default()
                    },
                    TextLayout::new_with_justify(JustifyText::Left),
                    Transform::from_xyz(0., 15., 0.),
                    bevy::sprite::Anchor::TopLeft,
                )],
            ))
            .insert(ChildOf(hud));
    }

    // commands.spawn((
    //     Hud,
    //     Mesh2d(meshes.add(Annulus::new(60., 61.))),
    //     MeshMaterial2d(materials.add(Color::srgba(0., 0.3, 0., 0.1))),
    //     children![
    //         (
    //             HudText::Flux,
    //             Sprite {
    //                 rect: Some(Rect::new(0., 0., 50., 5.)),
    //                 color: Color::srgba(1., 0.5, 0., 0.2),
    //                 anchor: bevy::sprite::Anchor::TopLeft,
    //                 ..default()
    //             },
    //             Transform::from_xyz(90., 150., 0.),
    //             children![(
    //                 Text2d::new("flux: 199"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(0., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::Capacity,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 125., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("capacity: 10"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::Cooldown,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 100., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("cooldown: 5"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::Hitpoint,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 75., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("hitpoint: 528"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::Speed,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 50., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("speed: 1"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::Torque,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 25., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("torque: 1"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::BkDistance,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., 0., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("bk.dst: 50"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::BkSpeed,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., -25., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("bk.speed: 1"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //         (
    //             HudText::BkTorque,
    //             Mesh2d(meshes.add(Rectangle::new(100., 5.))),
    //             MeshMaterial2d(materials.add(Color::srgba(0., 0.5, 0., 0.2))),
    //             Transform::from_xyz(140., -50., 0.),
    //             bevy::sprite::Anchor::TopLeft,
    //             children![(
    //                 Text2d::new("bk.torque: 1"),
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraCodeNerdFontMono-Light.ttf"),
    //                     font_size: 10.,
    //                     ..default()
    //                 },
    //                 TextLayout::new_with_justify(JustifyText::Left),
    //                 Transform::from_xyz(-50., 15., 0.),
    //                 bevy::sprite::Anchor::TopLeft,
    //             )],
    //         ),
    //     ],
    // ));
}
