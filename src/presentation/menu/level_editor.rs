use super::states::MenuState;
use crate::app::actions::action_axis_xy;
use crate::app::actions::ActionPrompt;
use crate::app::actions::EditorActions;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::master::level::current::LevelCommand;
use crate::gameplay::master::level::data::*;
use crate::gameplay::master::level::spawn::SpawnObject;
use crate::gameplay::utils::pos_to_tile;
use crate::gameplay::utils::pos_to_tile_center;
use crate::presentation::objects::WorldCameraBundle;
use crate::presentation::AdvancedGizmos;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy_egui::*;
use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use itertools::Itertools as _;
use leafwing_input_manager::action_state::ActionState;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EditorEnabled>()
            .init_resource::<Editor>()
            .add_systems(Startup, load_editor_tools)
            .add_systems(OnEnter(MenuState::LevelEditor), enable_editor)
            .add_systems(OnExit(EditorEnabled::Yes), delete_editor_camera)
            .add_systems(
                Update,
                (
                    save_editor_tools.run_if(in_state(EditorEnabled::Yes)),
                    draw_editor_menu.run_if(in_state(MenuState::LevelEditor)),
                    (
                        (update_cursor_point, select_objects, tool_input).chain(),
                        draw_tool_info,
                        highlight_selected_tile,
                        draw_labels,
                        //
                        spawn_editor_camera,
                        editor_camera_input,
                    )
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

fn enable_editor(mut next_editor_state: ResMut<NextState<EditorEnabled>>) {
    next_editor_state.set(EditorEnabled::Yes);
}

#[derive(Resource, Default)]
struct Editor {
    unsaved_changes: bool,

    /// Objects on selected tile
    selected: Vec<(Entity, LevelObjectId)>,

    /// World position where cursor points to
    world_cursor: Vec2,
}

#[derive(Resource, Default, Serialize, Deserialize)]
#[serde(default)]
struct EditorTools {
    add_object: LevelObject,
    snap_to_tile: bool,
    draw_labels: bool,
}

const EDITOR_TOOLS_USERDATA: &str = "editor_tools";
const EDITOR_TOOLS_SAVE_DELAY: Duration = Duration::from_secs(2);

fn load_editor_tools(mut commands: Commands, userdata: Res<Userdata>) {
    commands.insert_resource(userdata.read_and_update::<EditorTools>(EDITOR_TOOLS_USERDATA));
}

fn save_editor_tools(
    tools: Res<EditorTools>,
    userdata: Res<Userdata>,
    mut pending_save: Local<Option<Duration>>,
    time: Res<Time<Real>>,
) {
    if tools.is_changed() {
        pending_save.get_or_insert(time.elapsed());
    }

    if let Some(was) = *pending_save {
        if time.elapsed().saturating_sub(was) > EDITOR_TOOLS_SAVE_DELAY {
            *pending_save = None;
            userdata.write(EDITOR_TOOLS_USERDATA, &*tools);
        }
    }
}

fn draw_editor_menu(
    mut egui_ctx: EguiContexts,
    mut next_editor_state: ResMut<NextState<EditorEnabled>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut level_commands: EventWriter<LevelCommand>,
    mut editor: ResMut<Editor>,
    mut level: ResMut<CurrentLevel>,
    mut commands: Commands,
    mut tools: ResMut<EditorTools>,
) {
    let editor = &mut *editor;
    let level = &mut level.data;

    EguiPopup {
        name: "draw_editor_menu",
        anchor: egui::Align2::LEFT_TOP,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        let mut changed = false;

        egui::ScrollArea::both().show(ui, |ui| {
            // level data

            ui.heading("LEVEL");

            text_field(ui, &mut changed, "Pretty name", &mut level.name);

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

            ui.label("Press ESC to toggle modes");
            ui.label("");

            // add object

            ui.heading("ADD OBJECT");

            ui.checkbox(&mut tools.snap_to_tile, "Snap to tile");

            ui.label(format!("Object: {:?}", tools.add_object.data));
            ui.collapsing("Properties", |ui| {
                for (name, object) in [
                    ("Script point", LevelObjectData::ScriptPoint(default())),
                    ("Wall", LevelObjectData::TerrainWall(default())),
                    ("Floor", LevelObjectData::TerrainFloor(default())),
                    ("Light", LevelObjectData::TerrainLight(default())),
                ] {
                    if ui.button(name).clicked() {
                        tools.add_object.data = object;
                    }
                }

                ui.group(|ui| {
                    edit_object(ui, &mut changed, None, &mut tools.add_object);
                });
            });
            ui.label("");

            // edit/remove selected objects

            ui.heading("SELECTED");

            for (entity, id) in editor.selected.iter().copied().sorted_by_key(|v| v.0) {
                ui.horizontal(|ui| {
                    if ui.button("Remove").clicked() {
                        level.remove_object(id);
                        commands.try_despawn_recursive(entity);
                        changed = true;
                    }

                    if let Some(object) = level.get_object_mut(id) {
                        if ui.button("Pick").clicked() {
                            tools.add_object = object.clone();
                        }

                        let text = format!("[{}] {:?}", object.align.symbol(), object.data);

                        egui::CollapsingHeader::new(text)
                            .id_source(entity)
                            .default_open(false)
                            .show(ui, |ui| {
                                edit_object(ui, &mut changed, id.into(), object);
                            });
                    }
                });
            }
        });

        if changed {
            editor.unsaved_changes = true;
        }
    });
}

fn text_field(ui: &mut egui::Ui, changed: &mut bool, name: &str, value: &mut String) {
    ui.horizontal(|ui| {
        *changed |= ui.text_edit_singleline(value).changed();
        ui.label(name);
    });
}

fn edit_object(
    ui: &mut egui::Ui,
    changed: &mut bool,
    id: Option<LevelObjectId>,
    object: &mut LevelObject,
) {
    ui.label(format!("Pos {}", object.pos));

    egui::Grid::new(format!("{id:?}_align")).show(ui, |ui| {
        let mut button = |ui: &mut egui::Ui, value| {
            *changed |= ui
                .radio_value(&mut object.align, value, format!("{value:?}"))
                .changed();
        };

        ui.label("");
        button(ui, LevelAlign::Top);
        ui.label("");
        ui.end_row();

        button(ui, LevelAlign::Left);
        button(ui, LevelAlign::Center);
        button(ui, LevelAlign::Right);
        ui.end_row();

        ui.label("");
        button(ui, LevelAlign::Bottom);
        ui.label("");
        ui.end_row();
    });

    match &mut object.data {
        LevelObjectData::ScriptPoint(object) => {
            ui.label("Script point");
            text_field(ui, changed, "ID", &mut object.id);

            ui.small("Set ID:");
            let mut id_button = |ui: &mut egui::Ui, value: &str| {
                if ui.button(value).clicked() {
                    object.id = value.to_string();
                }
            };
            ui.horizontal(|ui| {
                id_button(ui, "player");
            });
        }
        LevelObjectData::TerrainWall(_object) => {
            ui.label("Wall");
        }
        LevelObjectData::TerrainFloor(_object) => {
            ui.label("Floor");
        }
        LevelObjectData::TerrainLight(_object) => {
            ui.label("Light");
        }
        LevelObjectData::None => {
            ui.label("None");
        }
    }
}

//

fn select_objects(
    mut editor: ResMut<Editor>,
    objects: Query<(&GlobalTransform, Entity, &LevelObjectId)>,
) {
    let cursor_tile = pos_to_tile(editor.world_cursor);

    editor.selected = objects
        .iter()
        .filter_map(|(transform, entity, id)| {
            let entity_tile = pos_to_tile(transform.translation().xy());
            (entity_tile == cursor_tile).then_some((entity, *id))
        })
        .collect();
}

fn tool_input(
    actions: Res<ActionState<EditorActions>>,
    mut editor: ResMut<Editor>,
    mut level: ResMut<CurrentLevel>,
    mut commands: Commands,
    mut spawn_commands: EventWriter<SpawnObject>,
    mut tools: ResMut<EditorTools>,
) {
    let editor = &mut *editor;
    let level = &mut level.data;

    if actions.just_pressed(EditorActions::Tool)
        && !matches!(tools.add_object.data, LevelObjectData::None)
    {
        let pos = editor.world_cursor;
        let pos = match tools.snap_to_tile {
            true => pos_to_tile_center(pos),
            false => pos,
        };

        let mut object = tools.add_object.clone();
        object.pos = pos;

        let id = level.add_object(object.clone());
        spawn_commands.send(SpawnObject { id, object });

        editor.unsaved_changes = true;
    }

    if actions.pressed(EditorActions::ToolAlt) {
        for (entity, id) in editor.selected.drain(..) {
            level.remove_object(id);
            commands.try_despawn_recursive(entity);
            editor.unsaved_changes = true;
        }
    }

    if actions.just_pressed(EditorActions::SwitchDisplay) {
        tools.draw_labels = !tools.draw_labels;
    }

    for (action, align) in [
        (EditorActions::AlignCenter, LevelAlign::Center),
        (EditorActions::AlignTop, LevelAlign::Top),
        (EditorActions::AlignBottom, LevelAlign::Bottom),
        (EditorActions::AlignLeft, LevelAlign::Left),
        (EditorActions::AlignRight, LevelAlign::Right),
    ] {
        if actions.just_pressed(action) {
            tools.add_object.align = align;
        }
    }
}

fn draw_tool_info(
    mut egui_ctx: EguiContexts,
    editor: Res<Editor>,
    prompt: ActionPrompt<EditorActions>,
    level: Res<CurrentLevel>,
    tools: Res<EditorTools>,
) {
    EguiPopup {
        name: "draw_tool_info",
        anchor: egui::Align2::LEFT_TOP,
        interactable: false,
        background: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.label(format!("Add object: [{}]", prompt.get(EditorActions::Tool)));
        ui.label(format!(
            "Remove all objects on tile: [{}]",
            prompt.get(EditorActions::ToolAlt)
        ));
        ui.label(format!(
            "Toggle labels: [{}]",
            prompt.get(EditorActions::SwitchDisplay)
        ));
        ui.label(format!("Set align: arrows & zero"));

        ui.label("");
        ui.label(format!(
            "Add object: [{}] {:?}",
            tools.add_object.align.symbol(),
            tools.add_object.data
        ));
        ui.label(format!("Snap to tile: {:?}", tools.snap_to_tile));

        if !editor.selected.is_empty() {
            ui.label("");
            ui.label("Selected objects:");

            for (_, id) in editor.selected.iter().sorted_by_key(|v| v.0) {
                if let Some(object) = level.data.get_object(*id) {
                    let text = format!("[{}] {:?}", object.align.symbol(), object.data);
                    ui.label(text);
                }
            }
        }
    });
}

fn highlight_selected_tile(editor: Res<Editor>, mut gizmos: Gizmos) {
    let pos = pos_to_tile_center(editor.world_cursor);
    gizmos.rect_2d(pos, 0., Vec2::splat(TILE_SIZE), Color::GREEN.with_a(0.4));
}

fn draw_labels(
    level: Res<CurrentLevel>,
    mut adv_gizmos: AdvancedGizmos,
    objects: Query<(&GlobalTransform, &LevelObjectId)>,
    tools: Res<EditorTools>,
) {
    if !tools.draw_labels {
        return;
    }

    for (transform, id) in objects.iter() {
        let Some(object) = level.data.get_object(*id) else { return; };

        let (index, text) = match &object.data {
            LevelObjectData::None => (0, "NONE".to_string()),
            LevelObjectData::ScriptPoint(object) => (1, format!("SP:{}", object.id)),
            LevelObjectData::TerrainWall(_) => (2, "Wall".to_string()),
            LevelObjectData::TerrainFloor(_) => (3, "Floor".to_string()),
            LevelObjectData::TerrainLight(_) => (4, "Light".to_string()),
        };
        let index_count = 6.; // +2 from max index

        let pos = pos_to_tile_center(transform.translation().truncate()) - HALF_TILE;
        let y = (index as f32 / index_count) * TILE_SIZE;

        adv_gizmos.world_text(pos + Vec2::Y * y, text);
    }
}

//

#[derive(Component)]
struct EditorCamera;

fn spawn_editor_camera(mut commands: Commands, camera: Query<(), With<EditorCamera>>) {
    if camera.is_empty() {
        commands.spawn((
            {
                let mut camera = WorldCameraBundle::new("editor camera");
                camera.camera.camera.order = 1; // after player camera
                camera
            },
            EditorCamera,
        ));
    }
}

fn delete_editor_camera(mut commands: Commands, camera: Query<Entity, With<EditorCamera>>) {
    for entity in camera.iter() {
        commands.try_despawn_recursive(entity);
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

fn update_cursor_point(
    mut editor: ResMut<Editor>,
    camera: Query<(&GlobalTransform, &Camera), With<EditorCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else { return; };
    let Some(cursor) = window.cursor_position() else { return; };

    let Ok((camera_pos, camera)) = camera.get_single() else { return; };

    let world_cursor = camera.viewport_to_world_2d(&camera_pos, cursor);

    editor.world_cursor = world_cursor.unwrap_or_default();
}
