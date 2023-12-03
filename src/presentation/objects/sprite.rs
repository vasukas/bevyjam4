use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::TILE_SIZE;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

/// Colored rectangle
#[derive(Component, Clone)]
pub struct SimpleSprite {
    pub color: Color,
    pub size: Vec2,
    pub z_offset: f32,
}

impl Default for SimpleSprite {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            size: Vec2::splat(TILE_SIZE),
            z_offset: 0.,
        }
    }
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_debug_sprite.in_set(SpawnSet::Details),
                update_debug_sprite,
            ),
        );
    }
}

#[derive(Component)]
struct SpriteChild(Entity);

fn spawn_debug_sprite(
    new: Query<(Entity, &SimpleSprite), Added<SimpleSprite>>,
    mut commands: Commands,
) {
    for (entity, sprite) in new.iter() {
        let sprite = sprite.clone();

        commands.try_command(entity, move |entity| {
            let id = entity.with_child(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: sprite.color,
                            custom_size: Some(sprite.size),
                            ..default()
                        },
                        transform: Transform::from_xyz(0., 0., sprite.z_offset),
                        ..default()
                    })
                    .id()
            });
            entity.insert(SpriteChild(id));
        });
    }
}

fn update_debug_sprite(
    changed: Query<(&SpriteChild, &SimpleSprite), Changed<SimpleSprite>>,
    mut sprites: Query<(&mut Sprite, &mut Transform)>,
) {
    for (child, simple) in changed.iter() {
        let Ok((mut sprite, mut transform)) = sprites.get_mut(child.0) else { return; };

        sprite.color = simple.color;
        sprite.custom_size = Some(simple.size);
        transform.translation.z = simple.z_offset;
    }
}
