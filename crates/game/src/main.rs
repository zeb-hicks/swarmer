// #![windows_subsystem = "windows"]

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(400.)),
        material: materials.add(ColorMaterial {
            color: Color::rgb(1.0, 1.0, 1.0),
            texture: None,
        }),
        ..default()
    });
}
