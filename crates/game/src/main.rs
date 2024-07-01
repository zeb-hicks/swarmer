#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use entities::EntityPlugin;
use input::InputPlugin;
use minions::MinionPlugin;
use player::PlayerPlugin;
use pixelate::PixelatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelatePlugin {
            width: 512,
            height: 288,
        })
        .add_plugins(PlayerPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(MinionPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
