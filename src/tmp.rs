use crate::app::assets::LoadedAllAssets;
use crate::app::assets::TrackAssets;
use crate::app::settings::AppSettings;
use crate::utils::for_crate::bevy_egui::*;
use bevy::audio::VolumeLevel;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;

pub struct TmpPlugin;

impl Plugin for TmpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TestAssets>()
            .add_systems(Startup, (set_global_volume, load_test_assets))
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

fn set_global_volume(mut global_volume: ResMut<GlobalVolume>) {
    global_volume.volume = VolumeLevel::new(0.5)
}

#[derive(Resource, Default)]
struct TestAssets {
    image: Handle<Image>,
    audio: Handle<AudioSource>,
}

fn load_test_assets(mut assets: ResMut<TestAssets>, mut track: TrackAssets) {
    *assets = TestAssets {
        image: track.load_and_track("test.png"),
        audio: track.load_and_track("test.ogg"),
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

    // more quads!
    let count = 110;
    let material = color_materials.add(ColorMaterial {
        color: Color::WHITE.with_a(0.5),
        texture: assets.image.clone().into(),
    });
    let size = Vec2::new(1280., 720.) * 0.85;
    let delta = size / count as f32;
    for y in 0..count {
        for x in 0..count {
            let pos = Vec2::new(x as f32, y as f32);
            let pos = pos * delta - size / 2.;
            let scale = delta.max_element() * 0.7;

            commands.spawn((
                ColorMesh2dBundle {
                    mesh: xy_quad.clone().into(),
                    material: material.clone(),
                    transform: Transform::from_translation(pos.extend(10.))
                        .with_scale(Vec3::splat(scale)),
                    ..default()
                },
                TestObject { dir: -1. },
            ));
        }
    }

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

    // sound
    commands.spawn(AudioBundle {
        source: assets.audio.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
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
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Right) || keys.just_pressed(KeyCode::R) {
        for mut object in objects.iter_mut() {
            object.dir *= -1.;
        }
    }

    if keys.just_pressed(KeyCode::F) {
        let mut window = window.get_single_mut().unwrap();

        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        };
    }

    EguiPopup {
        name: "test_input",
        anchor: egui::Align2::LEFT_BOTTOM,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        egui::Frame::popup(&ui.style()).show(ui, |ui| {
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
    });
}
