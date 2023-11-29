use crate::app::assets::LoadedAllAssets;
use crate::app::assets::TrackAssets;
use crate::app::settings::AppSettings;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct TmpPlugin;

impl Plugin for TmpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TestAssets>()
            .add_systems(Startup, load_test_assets)
            .add_systems(
                Update,
                (
                    test_spawn.run_if(on_event::<LoadedAllAssets>()),
                    test_update,
                    test_input,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct TestAssets {
    image: Handle<Image>,
}

fn load_test_assets(mut assets: ResMut<TestAssets>, mut track: TrackAssets) {
    *assets = TestAssets {
        image: track.load_and_track("test.png"),
    };
}

fn test_spawn(
    mut commands: Commands,
    assets: Res<TestAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let xy_quad = meshes.add(shape::Quad::default().into());

    // quad with test image
    commands.spawn((
        ColorMesh2dBundle {
            mesh: xy_quad.clone().into(),
            material: color_materials.add(ColorMaterial {
                color: Color::rgb(3., 1., 3.),
                texture: assets.image.clone().into(),
            }),
            transform: Transform::from_xyz(0., 120., 0.).with_scale(Vec3::splat(600.)),
            ..default()
        },
        TestObject { dir: 1. },
    ));

    // 2d camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            deband_dither: DebandDither::Enabled,
            ..default()
        },
        BloomSettings::OLD_SCHOOL,
    ));
}

#[derive(Component)]
struct TestObject {
    dir: f32,
}

fn test_update(mut objects: Query<(&mut Transform, &TestObject)>, time: Res<Time>) {
    for (mut transform, object) in objects.iter_mut() {
        let speed = 180_f32.to_radians();
        let delta = speed * time.delta_seconds() * object.dir;

        transform.rotation *= Quat::from_rotation_z(delta);
    }
}

fn test_input(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut objects: Query<&mut TestObject>,
    mut egui_ctx: EguiContexts,
    mut settings: ResMut<AppSettings>,
) {
    if buttons.just_pressed(MouseButton::Right) || keys.just_pressed(KeyCode::R) {
        for mut object in objects.iter_mut() {
            object.dir *= -1.;
        }
    }

    egui::Area::new("test_input")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::ZERO)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("A text");

            if ui.button("Print error to log").clicked() {
                error!("error test");
            }
            if ui.button("Print info to log").clicked() {
                error!("info test");
            }

            //

            let m_settings = settings.bypass_change_detection();
            let mut changed = false;

            changed |= ui
                .checkbox(&mut m_settings.log.show_all, "log.show_all")
                .changed();
            changed |= ui
                .checkbox(&mut m_settings.log.show_errors, "log.show_errors")
                .changed();

            if changed {
                settings.set_changed();
            }
        });
}
