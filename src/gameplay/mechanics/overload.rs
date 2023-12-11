use super::damage::Dead;
use crate::gameplay::balance::OVERLOAD_RADIUS;
use crate::gameplay::physics::*;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct OverloadSource {
    pub power: f32,
}

/// Basically health, affected by nearby entities with [`OverloadSource`]
#[derive(Component)]
pub struct Overload {
    pub max: f32,
    pub current: f32,
}

impl Overload {
    pub fn new(max_load: f32) -> Self {
        Self {
            max: max_load,
            current: 0.,
        }
    }
}

#[derive(Component)]
pub struct MagicRecepient {
    pub count: usize,
}

#[derive(Component)]
pub struct MagicEmitter;

pub struct OverloadPlugin;

impl Plugin for OverloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_overload, boss_magic));
    }
}

fn boss_magic(
    mut added: Query<(Entity, &mut Overload), With<MagicEmitter>>,
    mut commands: Commands,
) {
    for (index, (entity, mut overload)) in added.iter_mut().enumerate() {
        let (count, load) = match index {
            0 => (4, 6.),
            1 => (3, 10.),
            2 => (2, 14.),
            3 => (1, 17.),
            _ => continue,
        };
        commands.try_insert(entity, MagicRecepient { count });
        overload.max = load;
    }
}

fn update_overload(
    mut overloads: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Overload,
            Option<&MagicRecepient>,
        ),
        Without<Dead>,
    >,
    sources: Query<&OverloadSource>,
    physics: Res<RapierContext>,
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    emitters: Query<(), (With<MagicEmitter>, Without<Dead>)>,
) {
    let frame_count = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
        .and_then(|diag| diag.value())
        .unwrap_or_default() as u32;

    let shape = Collider::ball(OVERLOAD_RADIUS);

    for (entity, pos, mut overload, magic) in overloads.iter_mut() {
        if entity.index() & 3 != frame_count & 3 {
            continue;
        }

        overload.current = 0.;

        physics.intersections_with_shape(
            pos.translation().truncate(),
            0.,
            &shape,
            PhysicsType::Overload.filter(),
            |entity| {
                if let Ok(source) = sources.get(entity) {
                    overload.current += source.power
                }
                true
            },
        );

        if let Some(magic) = magic {
            let emitters = emitters.iter().count();
            if emitters > magic.count {
                overload.current = 0.;
            }
        }

        if overload.current >= overload.max {
            commands.try_insert(entity, Dead);
        }
    }
}
