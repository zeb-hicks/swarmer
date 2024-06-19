use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
		app.add_systems(Update, update);
    }
}


fn update() {}
fn setup() {}


#[derive(Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32
}

#[derive(Component)]
pub struct KinematicEntity {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

#[derive(Component)]
pub struct MovementDirect {
    pub input_movement: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub max_speed: f32,
}
