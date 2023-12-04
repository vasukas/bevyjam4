use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy::misc_utils::iterate_children_recursively;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use std::collections::BTreeMap;
use std::time::Duration;

/// Animation controller for GLTF-spawned scene with single animated object.
///
/// Plays highest priority active animation.
#[derive(Component, Default)]
pub struct AnimationCtl {
    layers: BTreeMap<usize, Layer>,
    playing: Option<usize>,
    restart: bool,
}

#[derive(Default)]
struct Layer {
    clip: Handle<AnimationClip>,
    repeat: bool,
    speed: f32,
    active: bool,
}

impl AnimationCtl {
    /// Bigger layer index - higher priority.
    ///
    /// There MUST be layer with id 0 and it's always active.
    pub fn layer(
        &mut self,
        id: impl Into<usize>,
        clip: Handle<AnimationClip>,
        repeat: bool,
        speed: f32,
    ) {
        let id = id.into();
        let layer = self.layers.entry(id).or_default();

        layer.clip = clip;
        layer.repeat = repeat;
        layer.speed = speed;
    }

    pub fn set_active(&mut self, id: impl Into<usize>, active: bool) {
        let id = id.into();
        let layer = self.layers.entry(id).or_default();

        if Some(id) == self.playing && active && layer.active && !layer.repeat {
            self.restart = true;
        } else {
            layer.active = active;
        }
    }
}

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AnimationCtlSystem;

pub struct AnimationCtlPlugin;

impl Plugin for AnimationCtlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                find_animation_player,
                disable_culling,
                update_controller
                    .before(bevy::animation::animation_player)
                    .in_set(AnimationCtlSystem),
            ),
        );
    }
}

const TRANSITION: Duration = Duration::from_millis(150);

#[derive(Component)]
struct AnimationPlayerRef(Entity);

fn find_animation_player(
    controllers: Query<Entity, (With<AnimationCtl>, Without<AnimationPlayerRef>)>,
    mut commands: Commands,
    children: Query<&Children>,
    players: Query<(), With<AnimationPlayer>>,
) {
    for root in controllers.iter() {
        let mut player = None;
        iterate_children_recursively(root, &children, |entity| {
            if players.contains(entity) {
                player = Some(entity);
            }
        });
        if let Some(player) = player {
            commands.try_insert(root, AnimationPlayerRef(player));
        }
    }
}

// no idea if this is still required - in old versions animated models
// sometimes are culled too early when close to edge of the screen
fn disable_culling(
    controllers: Query<Entity, Added<AnimationPlayerRef>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Query<&Handle<Mesh>>,
) {
    for root in controllers.iter() {
        iterate_children_recursively(root, &children, |entity| {
            if meshes.contains(entity) {
                commands.try_insert(entity, NoFrustumCulling);
            }
        });
    }
}

fn update_controller(
    mut controllers: Query<(&mut AnimationCtl, &AnimationPlayerRef)>,
    mut players: Query<&mut AnimationPlayer>,
    clips: Res<Assets<AnimationClip>>,
) {
    for (mut ctl, player) in controllers.iter_mut() {
        let Ok(mut player) = players.get_mut(player.0) else { continue; };

        let mut start = false;

        // restart current animation
        if std::mem::take(&mut ctl.restart) {
            start = true;
        }

        // check current non-repeating animation has ended
        if let Some(id) = ctl.playing {
            let layer = ctl.layers.get_mut(&id).unwrap();
            if !layer.repeat {
                let clip = player.animation_clip();
                let clip_duration = clips.get(clip).map(|clip| clip.duration()).unwrap_or(0.);

                if player.elapsed() >= clip_duration {
                    layer.active = false;
                    ctl.playing = None;
                }
            }
        }

        // change current animation
        let new_playing = ctl
            .layers
            .iter()
            .filter_map(|(id, layer)| layer.active.then_some(*id))
            .max()
            .or(Some(0));
        if new_playing != ctl.playing {
            ctl.playing = new_playing;
            start = true;
        }

        // start playback
        if start {
            let id = ctl.playing.unwrap_or(0);
            let layer = ctl.layers.get_mut(&id).unwrap();

            player.start_with_transition(layer.clip.clone(), TRANSITION);
            if layer.repeat {
                player.repeat();
            }
            player.set_speed(layer.speed);
        }
    }
}
