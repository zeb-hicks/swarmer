use assets::sprite_sheet_bundle;
use bevy::prelude::*;
use entities::{GlobalResources, KinematicEntity, MovementPathing, MovementType};
use input::GameInputIntent;
use pixelate::PIXEL_LAYER;
use rand::prelude::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
		app.add_systems(Update, (
            count_minions,
            spawn_minions,
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
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
            let x = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius);
            let y = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius);
            let tile = match spawner.minion_type {
                MinionType::Melee1 => 98,
                MinionType::Melee2 => 96,
                MinionType::Ranged1 => 112,
                MinionType::Ranged2 => 111,
            };

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
                sprite_sheet_bundle(&asset_server, &mut atlas_layouts, Transform::from_xyz(x, y, -y), tile),
                KinematicEntity {
                    position: Vec2::new(x, y) + transform.translation.xy(),
                    velocity: Vec2::new(0.0, 0.0),
                    radius: 4.0,
                },
                Minion {
                    minion_type: spawner.minion_type,
                },
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

fn move_minions(
    // mut commands: Commands,
    // time: Res<Time>,
    mut minions: Query<(Entity, &KinematicEntity, &mut MovementPathing, &mut Transform)>,
) {
    for (_entity, kin, mut movement, mut _transform) in minions.iter_mut() {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) {
            let x = rng.gen_range(-32.0..32.0);
            let y = rng.gen_range(-32.0..32.0);
            movement.target = Some(Vec2::new(x, y) + kin.position);
        }
    }
}