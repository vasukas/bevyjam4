use super::states::CloseMenu;
use super::states::MenuState;
use crate::app::actions::ActionPrompt;
use crate::app::actions::AppActions;
use crate::utils::bevy_egui::*;
use crate::utils::plugins::load_assets::TrackAssets;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use leafwing_input_manager::action_state::ActionState;

pub struct InoutroPlugin;

impl Plugin for InoutroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(
                Update,
                ((escape, show_text)
                    .run_if(in_state(MenuState::Intro).or_else(in_state(MenuState::Outro))),),
            )
            .add_systems(OnEnter(MenuState::Intro), spawn_background)
            .add_systems(OnEnter(MenuState::Outro), spawn_background)
            .add_systems(OnExit(MenuState::Intro), despawn_background)
            .add_systems(OnExit(MenuState::Outro), despawn_background);
    }
}

fn escape(
    mut egui_ctx: EguiContexts,
    app_prompt: ActionPrompt<AppActions>,
    mut close_menu: EventWriter<CloseMenu>,
    actions: Res<ActionState<AppActions>>,
) {
    EguiPopup {
        name: "show_escape",
        anchor: egui::Align2::RIGHT_BOTTOM,
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            ui.label(format!(
                "Press {} to skip",
                app_prompt.get(AppActions::CloseMenu)
            ));
        });
    });

    if actions.just_pressed(AppActions::Continue) {
        close_menu.send_default();
    }
}

fn show_text(
    mut egui_ctx: EguiContexts,
    app_prompt: ActionPrompt<AppActions>,
    state: Res<State<MenuState>>,
) {
    EguiPopup {
        name: "intro",
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        let style = ui.style_mut();
        style.visuals.window_fill = Color::BLACK.with_a(0.5).to_egui();
        style.visuals.window_stroke = egui::Stroke::NONE;
        style.visuals.override_text_color = Some(Color::WHITE.to_egui());

        egui::Frame::popup(style).show(ui, |ui| {
            match state.get() {
                MenuState::Intro => {
                    ui.label(concat!(
                        "Jim was walking in a forest, when bright light appeared in the sky.\n",
                        "Next thing he remembers, he is on an alien ship!\n",
                        "\n",
                        "Something has happened to Jim, now he can create fire and move\n",
                        "objects with his will!\n",
                        "\n",
                        "Using his newfound powers, Jim escapes from his cell...\n",
                    ));
                }
                MenuState::Outro => {
                    ui.label(concat!(
                        "Jim reached the ship's bridge.\n",
                        "No one was where, just lights on console blinking in total silence.\n",
                        "\n",
                        "After some trial and error Jim had figured the controls\n",
                        "and set course back to Earth.\n",
                    ));
                    ui.heading("Thank you for playing!\n");
                }
                _ => error!("invalid state"),
            }

            ui.label(format!(
                "[Press {} or {} to continue]",
                app_prompt.get(AppActions::Continue),
                app_prompt.get(AppActions::CloseMenu)
            ));
        });
    });
}

#[derive(Resource)]
struct MenuAssets {
    intro: Handle<Image>,
    outro: Handle<Image>,
}

fn load_assets(mut track: TrackAssets, mut commands: Commands) {
    commands.insert_resource(MenuAssets {
        intro: track.load_and_track("forest.png"),
        outro: track.load_and_track("bridge.png"),
    })
}

#[derive(Component)]
struct Background;

fn spawn_background(assets: Res<MenuAssets>, mut commands: Commands, state: Res<State<MenuState>>) {
    let texture = match state.get() {
        MenuState::Intro => assets.intro.clone(),
        MenuState::Outro => assets.outro.clone(),
        _ => {
            error!("invalid state");
            return;
        }
    };

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1000,
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(720.),
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
        Background,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., -1.),
            texture,
            ..default()
        },
        Background,
    ));
}

fn despawn_background(entities: Query<Entity, With<Background>>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
