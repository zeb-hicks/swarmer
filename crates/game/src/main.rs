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

pub enum MinionType {
    Melee1,
    Melee2,
    Ranged1,
    Ranged2,
}

#[derive(Component)]
pub struct MinionSpawner {
    pub spawn_rate: f32,
    pub spawn_timer: f32,
    pub spawn_radius: f32,
    pub spawn_limit: i32,
    pub minion_type: MinionType,
}

fn spawn_minions(
    // spawners: Query()
) {

}
