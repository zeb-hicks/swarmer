use assets::sprite_sheet_bundle;
use bevy::{prelude::*, utils::HashMap};
use input::GameInputIntent;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

use entities::{
    GlobalResources, Health, KinematicEntity, MovementDirect
};

#[derive(Resource)]
pub struct AssetHandles {
    pub images: HashMap<String, Handle<Image>>,
    pub layouts: HashMap<String, Handle<TextureAtlasLayout>>,
}

use minions::MinionSpawner;
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
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut globals: ResMut<GlobalResources>,
) {
    let player = commands.spawn((
        Player,
        sprite_sheet_bundle(&asset_server, &mut atlas_layouts, Transform::from_xyz(0., 0., 0.), 12 * 7),
        Health {
            max_health: 100.0,
            health: 100.0,
        },
        KinematicEntity {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 8.0,
        },
        MovementDirect {
            input_movement: Vec2::new(0.0, 0.0),
            direction: Vec2::new(1.0, 0.0),
            speed: 0.0,
            max_speed: 100.0,
        },
        MinionSpawner {
            spawn_timer: 0.0,
            spawn_delay: 0.01,
            spawn_radius: 100.0,
            spawn_limit: -1,
            minion_type: minions::MinionType::Melee1,
        },
        PIXEL_LAYER
    ));

    globals.player_entity = Some(player.id());
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

        *transform = Transform::from_translation(Vec3::new(kine.position.x.round(), kine.position.y.round(), -kine.position.y)) * Transform::from_rotation(Quat::from_rotation_z(look_angle));
    }
}
