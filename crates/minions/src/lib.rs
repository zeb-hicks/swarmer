use bevy::prelude::*;
use entities::{GlobalResources, MovementPathing, MovementType, SpatialEntity};
use input::GameInputIntent;
use pixelate::PIXEL_LAYER;
use rand::prelude::*;
use avian2d::prelude::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
		app.add_systems(Update, (
            count_minions,
            spawn_minions,
            minion_wander,
            move_minions,
        ));
        app.insert_resource(MinionStats::default());
    }
}

#[derive(Component)]
pub struct Minion {
    pub minion_type: MinionType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MinionType {
    Melee1,
    Melee2,
    Ranged1,
    Ranged2,
}

#[derive(Component)]
pub struct MinionSpawner {
    pub spawn_timer: f32,
    pub spawn_delay: f32,
    pub spawn_radius: f32,
    pub spawn_limit: i32,
    pub minion_type: MinionType,
}

#[derive(Resource, Default)]
pub struct MinionStats {
    pub total: u32,
    pub melee: u32,
    pub ranged: u32,
    pub army_strength: f32,
}

fn count_minions(
    minions: Query<&Minion>,
    mut minion_stats: ResMut<MinionStats>,
) {
    minion_stats.total = minions.iter().count() as u32;
}

fn spawn_minions(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    minion_stats: Res<MinionStats>,
    globals: Res<GlobalResources>,
    // mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // mut cached_layouts: ResMut<AtlasLayouts>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut MinionSpawner)>,
    inputs: Query<&GameInputIntent>,
) {
    for (transform, mut spawner) in query.iter_mut() {
        if !inputs.single().spawn { continue };
        if minion_stats.total > 2048 { continue };
        spawner.spawn_timer -= time.delta_seconds();
        if spawner.spawn_timer <= 0.0 && spawner.spawn_limit != 0 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius) + transform.translation.x;
            let y = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius) + transform.translation.y;
            // let tile = match spawner.minion_type {
            //     MinionType::Melee1 => 98,
            //     MinionType::Melee2 => 96,
            //     MinionType::Ranged1 => 112,
            //     MinionType::Ranged2 => 111,
            // };

            let mut movement = MovementPathing {
                max_speed: 60.0,
                speed: 0.0,
                movement_type: MovementType::Direct,
                ..default()
            };

            if let Some(player) = globals.player_entity {
                if rng.gen_bool(0.5) {
                    movement.target_entity = Some(player);
                }
            }

            let _minion = commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("soldier.png"),
                    transform: Transform::from_xyz(x, y, -y),
                    ..default()
                },
                Circle::new(4.0).collider(),
                RigidBody::Dynamic,
                Friction::new(0.1),
                LockedAxes::ROTATION_LOCKED,
                Minion {
                    minion_type: spawner.minion_type,
                },
                SpatialEntity,
                movement,
                PIXEL_LAYER
            ));
            spawner.spawn_timer += spawner.spawn_delay;
            if spawner.spawn_limit > 0 {
                spawner.spawn_limit -= 1;
            }
        }
    }
}

fn minion_wander(
    // mut commands: Commands,
    // time: Res<Time>,
    mut minions: Query<(Entity, &mut MovementPathing, &mut Transform)>,
) {
    minions.par_iter_mut().for_each(|(_, mut movement, mut _transform)| {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) {
            let x = rng.gen_range(-32.0..32.0);
            let y = rng.gen_range(-32.0..32.0);
            movement.target = Some(Vec2::new(x, y) + _transform.translation.truncate());
        }
    });
}

fn move_minions(
    mut minions: Query<(&mut LinearVelocity, &MovementPathing, &Transform)>,
) {
    minions.par_iter_mut().for_each(|(mut linear_velocity, movement, transform)| {
        let offset = match movement.target {
            Some(target) => target - transform.translation.truncate(),
            None => Vec2::ZERO,
        };
        let distance = offset.length();
        let speed = movement.max_speed;
        let direction = offset.normalize_or_zero();
        let mut motion = direction * speed;
        if distance <= speed {
            motion = offset;
        }
        linear_velocity.0 = motion;
    });
}