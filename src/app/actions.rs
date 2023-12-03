use crate::gameplay::master::level::data::LevelAlign;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

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
    SwitchDisplay,
    //
    Align(LevelAlign),
}

impl EditorActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(VirtualDPad::wasd(), Self::Movement)
            .insert(MouseButton::Left, Self::Tool)
            .insert(MouseButton::Right, Self::ToolAlt)
            .insert(KeyCode::Space, Self::SwitchDisplay)
            //
            .insert(KeyCode::Up, Self::Align(LevelAlign::Top))
            .insert(KeyCode::Down, Self::Align(LevelAlign::Bottom))
            .insert(KeyCode::Left, Self::Align(LevelAlign::Left))
            .insert(KeyCode::Right, Self::Align(LevelAlign::Right))
            .insert(KeyCode::Numpad0, Self::Align(LevelAlign::Center))
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

/// Get prompt for action (which key/button to press)
#[derive(SystemParam)]
pub struct ActionPrompt<'w, A: Actionlike + 'static> {
    map: Res<'w, InputMap<A>>,
}

impl<'w, A: Actionlike + 'static> ActionPrompt<'w, A> {
    pub fn get(&self, action: A) -> String {
        let mut inputs = self.map.get(action).iter();
        let Some(input) = inputs.next() else { return "NOT SET".to_string(); };

        let mut text = match input {
            UserInput::Single(input) => match input {
                InputKind::Keyboard(input) => format!("{input:?} key"),
                InputKind::Mouse(input) => format!("{input:?} mouse button"),
                _ => format!("Single:{input:?}"),
            },
            UserInput::Chord(input) => format!("Chord:{input:?}"),
            UserInput::VirtualDPad(input) => {
                if input == &VirtualDPad::wasd() {
                    "W/A/S/D".to_string()
                } else {
                    format!("VirtualDPad:{input:?}")
                }
            }
            UserInput::VirtualAxis(input) => format!("VirtualAxis:{input:?}"),
        };

        let and_more = inputs.next().is_some();
        if and_more {
            text += "... and more";
        }

        text
    }
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
