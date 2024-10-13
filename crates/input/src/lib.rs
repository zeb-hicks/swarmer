use bevy::{app::AppExit, prelude::*};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Spawn,
    Ability,
    Menu,
}

impl PlayerActions {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);
        input_map.insert_dual_axis(Self::Look, GamepadStick::RIGHT);
        input_map.insert_dual_axis(Self::Look, KeyboardVirtualDPad::ARROW_KEYS);
        input_map.insert(Self::Spawn, KeyCode::Space);
        input_map.insert(Self::Spawn, GamepadButtonType::RightTrigger);
        input_map.insert(Self::Menu, KeyCode::Escape);
        input_map.insert(Self::Menu, GamepadButtonType::Select);

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
    mut exit: EventWriter<AppExit>,
) {
    for (mut intent, actions) in &mut bundle {
        if actions.just_pressed(&PlayerActions::Menu) {
            exit.send_default();
        }

        let movement = actions.clamped_axis_pair(&PlayerActions::Move);
        let look = actions.clamped_axis_pair(&PlayerActions::Look);

        intent.movement = movement.xy().normalize_or_zero();
        intent.movement_amplitude = movement.xy().length();
        intent.look = look.xy().normalize_or_zero();
        intent.target = None;
        intent.target_entity = None;
        intent.spawn = actions.pressed(&PlayerActions::Spawn);
    }
}