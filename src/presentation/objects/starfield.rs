use super::assets::ObjectAssets;
use super::WorldCameraBundle;
use crate::app::settings::AppSettings;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::utils::Lifetime;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::misc_utils::ExtendedTime;
use crate::utils::random::RandomRange;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_stars);
    }
}

#[derive(Component)]
struct Star;

fn update_stars(
    mut stars: Query<(Entity, &mut Transform), With<Star>>,
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<ObjectAssets>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<&GlobalTransform, With<Camera3d>>,
    settings: Res<AppSettings>,
    level: Res<CurrentLevel>,
) {
    let allowed = settings.graphics.starfield && level.allow_starfield;

    if !allowed {
        for (entity, _) in stars.iter() {
            commands.try_despawn_recursive(entity);
        }
    }

    let period = Duration::from_millis(50);
    let speed = 5.;
    let distance = WorldCameraBundle::VERTICAL_SIZE + 15.;
    let lifetime = distance / speed;

    if time.is_tick(period, default()) && allowed {
        let aspect_ratio = window
            .get_single()
            .map(|window| window.width() / window.height())
            .unwrap_or(1.);
        let xdt = distance / 2. * aspect_ratio;
        let camera = camera
            .get_single()
            .map(|cam| cam.translation().truncate())
            .unwrap_or(Vec2::ZERO);

        for _ in 0..1 {
            let x = (-xdt..xdt).random();
            let scale = (0.05..0.1).random();
            commands.spawn((
                SceneBundle {
                    scene: assets.scene_star.clone(),
                    transform: Transform::from_xyz(x + camera.x, distance / 2. + camera.y, -1.)
                        .with_scale(Vec3::splat(scale)),
                    ..default()
                },
                Star,
                Lifetime(Duration::from_secs_f32(lifetime)),
            ));
        }
    }

    for (_, mut transform) in stars.iter_mut() {
        transform.translation.y -= speed * time.delta_seconds();
    }
}
