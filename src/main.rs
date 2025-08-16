use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = Image::default();

    commands.spawn((
        Sprite {
            image: images.add(image),
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn(Camera2d);
}
