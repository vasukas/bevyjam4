use super::assets::ObjectAssets;
use super::materials::Materials;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::objects::particles::Particle;
use crate::gameplay::utils::InterpolateTransformOnce;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct OtherObjectsPlugin;

impl Plugin for OtherObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (spawn_projectiles, spawn_particles).in_set(SpawnSet::Controllers),
        );
    }
}

fn spawn_projectiles(
    new: Query<(Entity, &Projectile), Added<Projectile>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
    materials: Res<Materials>,
) {
    for (entity, object) in new.iter() {
        let (material, scale) = match object.ty {
            DamageType::Player => (materials.projectile.clone(), 2.),
            DamageType::Barrels => (materials.fire_spark.clone(), 1.),
        };

        let bundle = PbrBundle {
            mesh: assets.mesh_sphere.clone(),
            material,
            transform: {
                let scale = object.radius * scale;
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
        let descr = object.descriptor();
        let scale = Vec3::splat(descr.graphical_size);
        let bundle = (
            PbrBundle {
                mesh: assets.mesh_sphere.clone(),
                material: match object {
                    Particle::ProjectileImpact => materials.projectile_impact.clone(),
                    Particle::FireImpact => materials.fire_spark.clone(),
                    Particle::ColdFire => materials.fire_cold.clone(),
                },
                transform: Transform::from_xyz(0., 0., 1.2).with_scale(scale),
                ..default()
            },
            InterpolateTransformOnce::new(descr.lifetime).scale(scale * 0.1),
        );

        commands.try_with_children(entity, |parent| {
            parent.spawn(bundle);
        });
    }
}
