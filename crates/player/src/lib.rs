use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

use entities::{
    MovementDirect,
    KinematicEntity,
    Health
};

use pixelate::PIXEL_LAYER;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
		app.add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Player,
        Health {
            max_health: 100.0,
            health: 100.0,
        },
        KinematicEntity {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        },
        MovementDirect {
            input_movement: Vec2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
            speed: 0.0,
            max_speed: 100.0,
        },
        SpriteBundle {
            texture: asset_server.load("tile_0084.png"),
            transform: Transform::from_xyz(0., 0., 1.),//.with_scale(Vec3::splat(8.0)),
            ..default()
        },
        PIXEL_LAYER
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Player, &mut KinematicEntity, &mut MovementDirect, &mut Transform)>,
    time: Res<Time>,
) {
    let mut axis = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::KeyA) { axis.x -= 1.; }
    if keyboard_input.pressed(KeyCode::KeyD) { axis.x += 1.; }
    if keyboard_input.pressed(KeyCode::KeyS) { axis.y -= 1.; }
    if keyboard_input.pressed(KeyCode::KeyW) { axis.y += 1.; }

    for (_, mut k, mut m, mut t) in &mut player {
        m.input_movement = axis;
        m.speed = m.input_movement.length() * m.max_speed;
        m.direction = m.input_movement.normalize_or_zero();

        k.velocity = m.direction * m.speed;
        let vel = k.velocity * time.delta_seconds();
        k.position += vel;

        t.translation = Vec3::new(k.position.x, k.position.y, 0.);
    }
}