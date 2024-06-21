use bevy::prelude::*;
use input::GameInputIntent;

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
        // app.add_plugins(InputManagerPlugin::<Action>::default());
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
            transform: Transform::from_xyz(0., 0., 1.).with_rotation(Quat::from_rotation_z(0.3)),//.with_scale(Vec3::splat(8.0)),
            ..default()
        },
        PIXEL_LAYER
    ));
}

fn player_movement(
    inputs: Query<&GameInputIntent>,
    mut player: Query<(&Player, &mut KinematicEntity, &mut MovementDirect, &mut Transform)>,
    time: Res<Time>,
) {
    let input = inputs.single();
    for (_, mut kine, mut movement, mut transform) in &mut player {
        let in_move = input.movement;

        let look = input.look;
        let look_angle = look.xy().to_angle();

        movement.input_movement = in_move;
        movement.speed = input.movement_amplitude * movement.max_speed;
        movement.direction = movement.input_movement.normalize_or_zero();

        kine.velocity = movement.direction * movement.speed;
        let vel = kine.velocity * time.delta_seconds();
        kine.position += vel;

        transform.translation = Vec3::new(kine.position.x.round(), kine.position.y.round(), 0.);
        transform.rotation = Quat::from_rotation_z(look_angle);
    }
}
