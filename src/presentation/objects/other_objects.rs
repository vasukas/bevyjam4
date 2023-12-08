use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use super::utils::MaterialCache;
use super::utils::ParticleMaterial;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level_progress::ExitUnlocked;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::objects::elevators::Elevator;
use crate::gameplay::objects::particles::Particle;
use crate::gameplay::objects::terrain::TerrainLight;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct OtherObjectsPlugin;

impl Plugin for OtherObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_materials).add_systems(
            PostUpdate,
            (
                spawn_projectiles,
                spawn_particles,
                spawn_elevators,
                unlock_exit_elevator.run_if(on_event::<ExitUnlocked>()),
            )
                .in_set(SpawnSet::Controllers),
        );
    }
}

#[derive(Resource)]
struct Materials {
    projectile: Handle<StandardMaterial>,
    projectile_impact: Handle<StandardMaterial>,
}

fn make_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: ResMut<MaterialCache>,
    mut commands: Commands,
) {
    commands.insert_resource(Materials {
        projectile: cache.get(
            &mut materials,
            ParticleMaterial::Simple {
                color: Color::rgb(0.7, 1.5, 2.),
            },
        ),
        projectile_impact: cache.get(
            &mut materials,
            ParticleMaterial::Simple {
                color: Color::CYAN.with_a(0.5),
            },
        ),
    });
}

fn spawn_projectiles(
    new: Query<(Entity, &Projectile), Added<Projectile>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
    materials: Res<Materials>,
) {
    for (entity, object) in new.iter() {
        let bundle = PbrBundle {
            mesh: assets.mesh_sphere.clone(),
            material: materials.projectile.clone(),
            transform: {
                let scale = object.radius * 2.;
                Transform::from_xyz(0., 0., 0.8).with_scale(Vec3::new(scale * 1.5, scale, scale))
            },
            ..default()
        };

        commands.try_with_children(entity, |parent| {
            parent.spawn(bundle);
        });
    }
}

fn spawn_particles(
    new: Query<(Entity, &Particle), Added<Particle>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
    materials: Res<Materials>,
) {
    for (entity, object) in new.iter() {
        let bundle = PbrBundle {
            mesh: assets.mesh_sphere.clone(),
            material: match object {
                Particle::ProjectileImpact => materials.projectile_impact.clone(),
            },
            transform: Transform::from_xyz(0., 0., 1.2),
            ..default()
        };

        commands.try_with_children(entity, |parent| {
            parent.spawn(bundle);
        });
    }
}

fn spawn_elevators(
    new: Query<(Entity, &Elevator), Added<Elevator>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let bundle = SceneBundle {
            scene: match object {
                Elevator::Enter => assets.scene_elevator_enter.clone(),
                Elevator::Exit => assets.scene_elevator_exit.clone(),
            },
            transform: rotate_3to2_tr(),
            ..default()
        };

        let bundle2 = elevator_lamp_bundle(object, false);

        commands.try_command(entity, |entity| {
            let id = entity.with_child(|parent| {
                parent.spawn(bundle);
                parent.spawn(bundle2).id()
            });
            entity.insert(ElevatorLamp(id));
        });
    }
}

#[derive(Component)]
struct ElevatorLamp(Entity);

fn unlock_exit_elevator(
    elevators: Query<(Entity, &Elevator, &ElevatorLamp)>,
    mut commands: Commands,
) {
    for (entity, object, lamp) in elevators.iter() {
        if object == &Elevator::Exit {
            commands.try_despawn_recursive(lamp.0);

            let bundle2 = elevator_lamp_bundle(object, true);
            commands.try_with_children(entity, |parent| {
                parent.spawn(bundle2);
            });
        }
    }
}

fn elevator_lamp_bundle(elevator: &Elevator, unlocked: bool) -> impl Bundle {
    (
        SpatialBundle::default(),
        TerrainLight::Custom {
            color: match (elevator, unlocked) {
                (Elevator::Enter, _) => Color::BLUE * 1.,
                (Elevator::Exit, false) => Color::RED * 2.,
                (Elevator::Exit, true) => Color::GREEN * 3.,
            },
            intensity: 50.,
            shadows: false,
        },
    )
}
