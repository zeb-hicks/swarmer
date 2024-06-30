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
            seperate_entities,
            update_transforms,
        ).chain());
        app.insert_resource(GlobalResources::default());
    }
}

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
    mut movables: Query<(&KinematicEntity, &mut MovementDirect)>,
) {
    for (mut _kin, mut _mov) in movables.iter_mut() {
        
    }
}

fn pathed_movement(
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicEntity, &Transform, Option<&mut MovementPathing>)>,
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
            let (_, target_kinematics, _, _) = target_enitity;
            target_positions.insert(*entity, target_kinematics.position);
        }
    }

    for (entity, mut kin, _, mov) in query.iter_mut() {
        if let Some(mut mov) = mov {
            match mov.movement_type {
                MovementType::Boid(ref mut _boid_data) => {
                    // gizmos.circle_2d(kin.position, kin.radius, Color::WHITE);
                    
                }
                MovementType::Direct => {
                    // gizmos.circle_2d(kin.position, kin.radius, Color::BLACK);
                    let mut target_pos = kin.position;
                    if let Some(_) = mov.target_entity {
                        if let Some(t_pos) = target_positions.get(&entity) {
                            target_pos = *t_pos;
                        }
                    } else if let Some(target) = mov.target {
                        target_pos = target;
                    }
    
                    let offset = target_pos - kin.position;
                    let distance = offset.length();
    
                    mov.speed = mov.max_speed;
                    let mov_dist = mov.speed * time.delta_seconds();
                    mov.direction = offset.normalize_or_zero();
    
                    if distance <= mov_dist {
                        kin.velocity = Vec2::ZERO;
                        kin.position = target_pos;
                        mov.speed = 0.0;
                    } else {
                        kin.velocity = offset.normalize_or_zero() * mov.speed;
                        let vel = kin.velocity * time.delta_seconds();
                        kin.position += vel;
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
    mut query: Query<(&KinematicEntity, &mut Transform)>,
) {
    for (kin, mut transform) in query.iter_mut() {
        *transform = Transform::from_translation(Vec3::new(kin.position.x.round(), kin.position.y.round(), -kin.position.y));
    }
}

fn seperate_entities(
    mut query: Query<(Entity, &mut KinematicEntity)>,
    globals: Res<GlobalResources>,
) {
    let mut pairs = query.iter_combinations_mut();
    let mut corrections: HashMap<Entity, Vec2> = HashMap::new();

    while let Some([a, b]) = pairs.fetch_next() {
        let (entity_a, kin_a) = a;
        let (entity_b, kin_b) = b;

        let offset = kin_a.position - kin_b.position;
        let distance = offset.length();
        let radius = kin_a.radius + kin_b.radius;

        if distance < radius {
            let overlap = radius - distance;
            let direction = offset.normalize_or_zero();
            let correction = direction * overlap / 2.0;

            let mut correction_a = correction;
            let mut correction_b = -correction;
            if let Some(c_a) = corrections.get(&entity_a) {
                correction_a += *c_a;
            }
            if let Some(c_b) = corrections.get(&entity_b) {
                correction_b += *c_b;
            } else {

            }

            if let Some(player) = globals.player_entity {
                if entity_a == player {
                    correction_b -= correction_a;
                    correction_a = Vec2::ZERO;
                }
                if entity_b == player {
                    correction_a -= correction_b;
                    correction_b = Vec2::ZERO;
                }
            }

            corrections.insert(entity_a, correction_a);
            corrections.insert(entity_b, correction_b);
        }
    }

    for (entity, mut kin) in query.iter_mut() {
        if let Some(correction) = corrections.get(&entity) {
            kin.position += *correction;
        }
    }
}