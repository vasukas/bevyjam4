use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum AppActions {
    Screenshot,
    CloseMenu,
    LevelEditor,
    Continue,
}

impl AppActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(KeyCode::F12, Self::Screenshot)
            .insert(KeyCode::Escape, Self::CloseMenu)
            .insert_chord([KeyCode::ControlLeft, KeyCode::E], Self::LevelEditor)
            .insert(KeyCode::Space, Self::Continue)
            .build()
    }
}

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum PlayerActions {
    Movement, // action_axis_xy
    ToggleHelp,
    Restart,
    Fire,
    Pull,
    Kick,
}

impl PlayerActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(VirtualDPad::wasd(), Self::Movement)
            .insert(KeyCode::I, Self::ToggleHelp)
            .insert(KeyCode::F1, Self::ToggleHelp)
            .insert(KeyCode::R, Self::Restart)
            .insert(MouseButton::Left, Self::Fire)
            .insert(MouseButton::Right, Self::Pull)
            .insert(KeyCode::K, Self::Fire)
            .insert(KeyCode::L, Self::Pull)
            .insert(KeyCode::F, Self::Kick)
            .build()
    }
}

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum EditorActions {
    Movement, // action_axis_xy
    Tool,
    ToolAlt,
    SwitchDisplay,
    UndoLastAdded,
    Pick,
    //
    AlignTop,
    AlignBottom,
    AlignLeft,
    AlignRight,
    AlignCenter,
    //
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl EditorActions {
    fn default_map() -> InputMap<Self> {
        InputMap::default()
            .insert(VirtualDPad::wasd(), Self::Movement)
            .insert(MouseButton::Left, Self::Tool)
            .insert(MouseButton::Right, Self::ToolAlt)
            .insert(KeyCode::Space, Self::SwitchDisplay)
            .insert(KeyCode::C, Self::UndoLastAdded)
            .insert(KeyCode::P, Self::Pick)
            //
            .insert(KeyCode::Up, Self::AlignTop)
            .insert(KeyCode::Down, Self::AlignBottom)
            .insert(KeyCode::Left, Self::AlignLeft)
            .insert(KeyCode::Right, Self::AlignRight)
            .insert(KeyCode::Numpad0, Self::AlignCenter)
            .insert(KeyCode::Key0, Self::AlignCenter)
            //
            .insert(KeyCode::Numpad8, Self::Rotate0)
            .insert(KeyCode::Numpad6, Self::Rotate90)
            .insert(KeyCode::Numpad2, Self::Rotate180)
            .insert(KeyCode::Numpad4, Self::Rotate270)
            //
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
        let mut text = String::new();

        for input in self.map.get(action).iter() {
            let s = match input {
                UserInput::Single(input) => match input {
                    InputKind::Keyboard(input) => format!("[{input:?} key]"),
                    InputKind::Mouse(input) => format!("[{input:?} mouse button]"),
                    _ => format!("[Single: {input:?}]"),
                },
                UserInput::Chord(input) => format!("[Chord: {input:?}]"),
                UserInput::VirtualDPad(input) => {
                    if input == &VirtualDPad::wasd() {
                        "[W/A/S/D]".to_string()
                    } else {
                        format!("[VirtualDPad: {input:?}]")
                    }
                }
                UserInput::VirtualAxis(input) => format!("[VirtualAxis: {input:?}]"),
            };

            if !text.is_empty() {
                text += " or ";
            }
            text += &s;
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
