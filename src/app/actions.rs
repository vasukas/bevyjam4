use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum AppActions {
    Screenshot,
    MenuBack,
}

impl AppActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(KeyCode::F12, Self::Screenshot)
            .insert(KeyCode::Escape, Self::MenuBack)
            .build()
    }
}

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum PlayerActions {
    Movement, // action_axis_xy
    PickUp,
    ThrowInteract,
}

impl PlayerActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(VirtualDPad::wasd(), Self::Movement)
            .insert(KeyCode::J, Self::PickUp)
            .insert(KeyCode::K, Self::ThrowInteract)
            .build()
    }
}

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum EditorActions {
    Movement, // action_axis_xy
    Tool,
    ToolAlt,
    ToolSwitch,
}

impl EditorActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(VirtualDPad::wasd(), Self::Movement)
            .insert(MouseButton::Left, Self::Tool)
            .insert(MouseButton::Right, Self::ToolAlt)
            .insert(KeyCode::Space, Self::ToolSwitch)
            .build()
    }
}

/// Normalized axis or zero
pub fn action_axis_xy<T: Actionlike>(state: &ActionState<T>, action: T) -> Vec2 {
    state
        .axis_pair(action)
        .and_then(|data| data.direction())
        .map(|dir| dir.unit_vector())
        .unwrap_or_default()
}

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<AppActions>::default(),
            InputManagerPlugin::<PlayerActions>::default(),
            InputManagerPlugin::<EditorActions>::default(),
        ))
        .init_resource::<ActionState<AppActions>>()
        .init_resource::<ActionState<PlayerActions>>()
        .init_resource::<ActionState<EditorActions>>()
        .insert_resource(AppActions::default_map())
        .insert_resource(PlayerActions::default_map())
        .insert_resource(EditorActions::default_map());
    }
}
