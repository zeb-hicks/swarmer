use bevy::prelude::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
		app.add_systems(Update, update);
    }
}

fn update() {}
fn setup() {}

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