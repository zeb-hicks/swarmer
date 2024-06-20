use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

use entities::{
    MovementDirect,
    KinematicEntity,
    Health
};

use pixelate::PIXEL_LAYER;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    Look,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Look, DualAxis::right_stick());

        input_map
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
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
        ActionState::<Action>::default(),
        Action::default_input_map(),
        // InputManagerBundle::with_map(Action::default_input_map()),
        PIXEL_LAYER
    ));
}

fn player_movement(
    mut player: Query<(&Player, &mut KinematicEntity, &mut MovementDirect, &mut Transform, &ActionState<Action>)>,
    time: Res<Time>,
) {
    for (_, mut k, mut m, mut t, a) in &mut player {
        let mut axis = Vec2::new(0., 0.);
        let axes = a.clamped_axis_pair(&Action::Move).unwrap();

        axis.x += axes.x();
        axis.y += axes.y();

        let look = a.clamped_axis_pair(&Action::Look).unwrap();
        let look_angle = Vec2::new(look.x(), look.y()).to_angle();

        m.input_movement = axis.normalize_or_zero();
        m.speed = m.input_movement.length() * m.max_speed;
        m.direction = m.input_movement.normalize_or_zero();

        k.velocity = m.direction * m.speed;
        let vel = k.velocity * time.delta_seconds();
        k.position += vel;

        t.translation = Vec3::new(k.position.x.round(), k.position.y.round(), 0.);
        t.rotation = Quat::from_rotation_z(look_angle);
    }
}