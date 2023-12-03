use super::states::MenuState;
use crate::app::actions::action_axis_xy;
use crate::app::actions::EditorActions;
use crate::gameplay::master::level::current::LevelCommand;
use crate::presentation::objects::WorldCameraBundle;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EditorEnabled>()
            .init_resource::<Editor>()
            .add_systems(
                Update,
                (
                    draw_editor_menu.run_if(in_state(MenuState::LevelEditor)),
                    (spawn_editor_camera, editor_camera_input)
                        .run_if(in_state(EditorEnabled::Yes).and_then(in_state(MenuState::None))),
                ),
            );
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditorEnabled {
    Yes,
    #[default]
    No,
}

#[derive(Resource, Default)]
struct Editor {
    unsaved_changes: bool,
}

fn draw_editor_menu(
    mut egui_ctx: EguiContexts,
    editor_state: Res<State<EditorEnabled>>,
    mut next_editor_state: ResMut<NextState<EditorEnabled>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut level_commands: EventWriter<LevelCommand>,
    mut editor: ResMut<Editor>,
) {
    if editor_state.get() == &EditorEnabled::No {
        next_editor_state.set(EditorEnabled::Yes);
    }

    EguiPopup {
        name: "draw_editor_menu",
        anchor: egui::Align2::LEFT_TOP,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        if editor.unsaved_changes {
            if ui.button("SAVE CHANGES").clicked() {
                editor.unsaved_changes = false;
                level_commands.send(LevelCommand::Save);
            }
        } else {
            let _ = ui.button("(no unsaved changes)");
        }
        if ui.button("Exit level editor").clicked() {
            next_menu_state.set(MenuState::MainMenu);
            next_editor_state.set(EditorEnabled::No);
        }
        ui.label("");

        //
    });
}

#[derive(Component)]
struct EditorCamera;

fn spawn_editor_camera(mut commands: Commands, camera: Query<(), With<EditorCamera>>) {
    if camera.is_empty() {
        commands.spawn((WorldCameraBundle::new("editor camera"), EditorCamera));
    }
}

fn editor_camera_input(
    actions: Res<ActionState<EditorActions>>,
    mut camera: Query<&mut Transform, With<EditorCamera>>,
    time: Res<Time<Real>>,
) {
    let speed = 20.;

    if let Ok(mut transform) = camera.get_single_mut() {
        let dir = action_axis_xy(&actions, EditorActions::Movement).extend(0.);

        transform.translation += dir * speed * time.delta_seconds();
    }
}
