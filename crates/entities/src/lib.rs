use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, utils::{HashMap, HashSet}};

#[derive(Component)]
pub struct Player {}

#[derive(Resource, Default)]
pub struct GlobalResources {
    pub player_entity: Option<Entity>,
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            direct_movement,
            pathed_movement,
            // seperate_entities,
            update_transforms,
        ).chain());
        // app.add_plugins(
        //     AutomaticUpdate::<SpatialEntity>::new()
        //         .with_spatial_ds(SpatialStructure::KDTree2)
        //         .with_frequency(Duration::from_millis(100)),
        // );
        app.insert_resource(GlobalResources::default());
    }
}

#[derive(Component)]
/** Marker component for entities tracked on the KD tree */
pub struct SpatialEntity;

pub struct PathData {
    pub path: Vec<Vec2>,
    pub next_node: usize,
    pub patrol: bool,
}

pub struct BoidData {
    pub seperation: Vec2,
    pub cohesion: Vec2,
    pub alignment: Vec2,
}

#[derive(Default)]
pub enum MovementType {
    #[default]
    Direct,
    Pathed(PathData),
    Boid(BoidData),
}

#[derive(Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32
}

#[derive(Component)]
pub struct MovementDirect {
    pub input_movement: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub max_speed: f32,
}

#[derive(Component, Default)]
pub struct MovementPathing {
    pub target: Option<Vec2>,
    pub target_entity: Option<Entity>,
    pub direction: Vec2,
    pub speed: f32,
    pub max_speed: f32,
    pub movement_type: MovementType,
}

#[derive(Component)]
pub struct Target {
    pub entity: Option<Entity>,
    pub position: Option<Vec2>,
}

pub struct SpatialGrid {
    pub grid: HashMap<(i16, i16), HashSet<Entity>>,
    pub associations: HashMap<Entity, (i16, i16)>,
}

fn direct_movement(
    mut movables: Query<(&Transform, &mut MovementDirect)>,
) {
    for (mut _kin, mut _mov) in movables.iter_mut() {
        
    }
}

fn pathed_movement(
    mut query: Query<(Entity, &mut Transform, &mut LinearVelocity, Option<&mut MovementPathing>)>,
    // mut gizmos: Gizmos,
) {
    let mut target_pairs: HashMap<Entity, Entity> = HashMap::new();
    let mut target_positions: HashMap<Entity, Vec2> = HashMap::new();

    // Store all the pairs of entities targeting other entities, and their target
    for (entity, _, _, mov) in query.iter_mut() {
        if let Some(mov) = mov {
            match mov.movement_type {
                MovementType::Direct => {
                    if let Some(target_entity) = mov.target_entity {
                        target_pairs.insert(entity, target_entity);
                    }
                }
                _ => {}
            }
        }
    }

    // Store the positions of each target entity
    for (entity, target) in target_pairs.iter() {
        if let Ok(target_enitity) = query.get(*target) {
            let (_, transform, _, _) = target_enitity;
            target_positions.insert(*entity, transform.translation.xy());
        }
    }

    for (entity, transform, mut linear_velocity, mov) in query.iter_mut() {
        if let Some(mut mov) = mov {
            match mov.movement_type {
                MovementType::Boid(ref mut _boid_data) => {
                    // gizmos.circle_2d(kin.position, kin.radius, Color::WHITE);
                }
                MovementType::Direct => {
                    // gizmos.circle_2d(kin.position, kin.radius, Color::BLACK);
                    let pos = transform.translation.xy();
                    let target;
                    if let Some(_) = mov.target_entity {
                        if let Some(t_pos) = target_positions.get(&entity) {
                            target = *t_pos;
                        } else {
                            target = pos;
                        }
                    } else if let Some(t) = mov.target {
                        target = t;
                    } else {
                        target = pos;
                    }

                    let diff = target - pos;
                    let dir = diff.normalize_or_zero();

                    let dist = diff.length_squared();
                    if dist < mov.speed * mov.speed {
                        linear_velocity.0 = diff;
                    } else {
                        linear_velocity.0 = dir * mov.speed;
                    }

                    // gizmos.arrow_2d(kin.position, target_pos, Color::RED);
                    // gizmos.arrow_2d(kin.position, kin.position + kin.velocity, Color::YELLOW);
                }
                MovementType::Pathed(ref mut _path_data) => {
                    // gizmos.circle_2d(kin.position, kin.radius, Color::BLUE);
    
                }
            }
        }
    }
}

fn update_transforms(
    mut query: Query<&mut Transform>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.z = -transform.translation.y;
    }
}
