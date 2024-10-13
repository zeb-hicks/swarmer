use bevy::{prelude::*, utils::HashMap};
use input::GameInputIntent;
use avian2d::prelude::*;
use entities::{
    GlobalResources, Health, MovementDirect, SpatialEntity
};
use minions::MinionSpawner;
use pixelate::PIXEL_LAYER;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct AssetHandles {
    pub images: HashMap<String, Handle<Image>>,
    pub layouts: HashMap<String, Handle<TextureAtlasLayout>>,
}

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
    // mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // mut cached_layouts: ResMut<AtlasLayouts>,
    mut globals: ResMut<GlobalResources>,
) {
    let player = commands.spawn((
        Player,
        // sprite_sheet_bundle(&asset_server, &mut atlas_layouts, Transform::from_xyz(0., 0., 0.), 12 * 7),
        SpriteBundle {
            texture: asset_server.load("wizard.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        // TextureAtlas {
        //     layout: get_atlas_layout(&mut atlas_layouts, &mut cached_layouts, 12 * 7),
        //     ..default()
        // },
        Health {
            max_health: 100.0,
            health: 100.0,
        },
        Circle::new(8.0).collider(),
        RigidBody::Kinematic,
        Friction::new(0.1),
        LockedAxes::ROTATION_LOCKED,
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
        SpatialEntity,
        PIXEL_LAYER
    ));

    globals.player_entity = Some(player.id());
}

fn player_movement(
    inputs: Query<&GameInputIntent>,
    mut player: Query<(&Player, &mut LinearVelocity, &mut MovementDirect)>,
) {
    let input = inputs.single();
    for (_, mut linear_velocity, mut movement) in &mut player {
        let in_move = input.movement;

        // let look = input.look;
        // let look_angle = look.xy().to_angle();

        movement.input_movement = in_move;
        movement.speed = input.movement_amplitude * movement.max_speed;
        movement.direction = movement.input_movement.normalize_or_zero();

        

        // linear_velocity = movement.direction * movement.speed;
        linear_velocity.0 = movement.direction * movement.speed;

        // *transform = Transform::from_translation(Vec3::new(kine.position.x.round(), kine.position.y.round(), -kine.position.y)) * Transform::from_rotation(Quat::from_rotation_z(look_angle));
    }
}
