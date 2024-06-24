use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    Move,
    Look,
    Spawn,
}

impl PlayerActions {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Spawn, GamepadButtonType::RightTrigger);

        input_map
    }
}

pub fn action_bundle() -> InputManagerBundle<PlayerActions> {
    InputManagerBundle::with_map(PlayerActions::default_input_map())
}

#[derive(Component, Default)]
pub struct GameInputIntent {
    pub movement: Vec2,
    pub movement_amplitude: f32,
    pub look: Vec2,
    pub target: Option<Vec2>,
    pub target_entity: Option<Entity>,
    pub spawn: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
        app.add_systems(Startup, setup_inputs);
		app.add_systems(Update, handle_inputs);
    }
}

fn setup_inputs(
    mut commands: Commands,
) {
    commands.spawn((
        GameInputIntent::default(),
        ActionState::<PlayerActions>::default(),
        PlayerActions::default_input_map(),
        // InputManagerBundle::with_map(Action::default_input_map()),
    ));
}

fn handle_inputs(
    mut bundle: Query<(&mut GameInputIntent, &ActionState<PlayerActions>)>,
) {
    for (mut intent, actions) in &mut bundle {
        let movement = actions.clamped_axis_pair(&PlayerActions::Move).unwrap();
        let look = actions.clamped_axis_pair(&PlayerActions::Look).unwrap();

        intent.movement = movement.xy().normalize_or_zero();
        intent.movement_amplitude = movement.xy().length();
        intent.look = look.xy().normalize_or_zero();
        intent.target = None;
        intent.target_entity = None;
        intent.spawn = actions.pressed(&PlayerActions::Spawn);
    }
}