// #![windows_subsystem = "windows"]

use bevy::prelude::*;
use player::PlayerPlugin;
use pixelate::PixelatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelatePlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, spawn_minions)
        .run();
}

fn spawn_minions(
    // spawners: Query()
) {

}
