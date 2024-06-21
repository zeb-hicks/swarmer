use bevy::prelude::*;
use rand::prelude::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
		app.add_systems(Update, spawn_minions);
    }
}

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

fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut MinionSpawner)>,
) {
    for (transform, mut spawner) in query.iter_mut() {
        spawner.spawn_timer -= time.delta_seconds();
        if spawner.spawn_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius);
            let y = rng.gen_range(-spawner.spawn_radius..spawner.spawn_radius);
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.0) + transform.translation),
                ..Default::default()
            });
            spawner.spawn_timer += spawner.spawn_delay;
        }
    }
}
