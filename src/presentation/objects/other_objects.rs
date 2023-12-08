use super::assets::ObjectAssets;
use super::utils::MaterialCache;
use super::utils::ParticleMaterial;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::objects::particles::Particle;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct OtherObjectsPlugin;

impl Plugin for OtherObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_materials).add_systems(
            PostUpdate,
            (spawn_projectiles, spawn_particles).in_set(SpawnSet::Controllers),
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
