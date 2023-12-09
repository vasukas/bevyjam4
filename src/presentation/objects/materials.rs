use super::assets::ObjectAssets;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub projectile: Handle<StandardMaterial>,
    pub projectile_impact: Handle<StandardMaterial>,
    pub fire_spark: Handle<StandardMaterial>,
    pub fire_cold: Handle<StandardMaterial>,
    // don't forget to add new ones to all() method!
}

impl Materials {
    fn all(&self) -> impl Iterator<Item = &Handle<StandardMaterial>> {
        [
            &self.projectile,
            &self.projectile_impact,
            &self.fire_spark,
            &self.fire_cold,
        ]
        .into_iter()
    }
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderCompilationHackState>()
            .add_systems(Startup, make_materials)
            .add_systems(
                Update,
                shader_compilation_hack.run_if(resource_exists::<ShaderCompilationHackState>()),
            );
    }
}

/// Descriptor for [`MaterialCache`]
#[derive(Clone, Copy)]
pub enum ParticleMaterial {
    /// Unlit, additive blending
    Simple { color: Color },
}

impl Into<StandardMaterial> for ParticleMaterial {
    fn into(self) -> StandardMaterial {
        match self {
            ParticleMaterial::Simple { color } => StandardMaterial {
                base_color: color,
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            },
        }
    }
}

fn make_materials(mut materials: ResMut<Assets<StandardMaterial>>, mut commands: Commands) {
    commands.insert_resource(Materials {
        projectile: materials.add(
            ParticleMaterial::Simple {
                color: Color::rgb(0.7, 1.5, 2.),
            }
            .into(),
        ),
        projectile_impact: materials.add(
            ParticleMaterial::Simple {
                color: Color::CYAN.with_a(0.5),
            }
            .into(),
        ),
        fire_spark: materials.add(
            ParticleMaterial::Simple {
                color: Color::rgb(3., 2.5, 1.5),
            }
            .into(),
        ),
        fire_cold: materials.add(
            ParticleMaterial::Simple {
                color: Color::ORANGE_RED.with_a(0.5),
            }
            .into(),
        ),
    });
}

#[derive(Component)]
struct ShaderCompilationHack;

#[derive(Resource, Default)]
struct ShaderCompilationHackState(u32);

// prevent shader stutter while in gameplay - do it only when camera is spawned
fn shader_compilation_hack(
    camera: Query<&GlobalTransform, With<Camera3d>>,
    hacks: Query<Entity, With<ShaderCompilationHack>>,
    mut commands: Commands,
    materials: Res<Materials>,
    assets: Res<ObjectAssets>,
    mut state: ResMut<ShaderCompilationHackState>,
) {
    let distance = 10.;
    let frame_count = 2;

    if hacks.is_empty() {
        let Ok(camera) = camera.get_single() else {return;};

        for material in materials.all() {
            let transform = {
                let mut trans = Transform::from(*camera);
                trans.translation += trans.forward() * distance;
                trans
            };
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_sphere.clone(),
                    material: material.clone(),
                    transform,
                    global_transform: transform.into(),
                    ..default()
                },
                ShaderCompilationHack,
            ));
        }
    } else {
        state.0 += 1;
        if state.0 < frame_count {
            return;
        }

        for entity in hacks.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<ShaderCompilationHackState>();
    }
}
