//! Fallible commands for [`bevy`]` crate.

use bevy::{ecs::system::EntityCommand, prelude::*};

/// Methods for [`Commands`] which fail instead of panicking
///
// Source: https://github.com/Leafwing-Studios/Emergence/pull/765/files#diff-bea0bd142041553803e04ece803b9e2306f73b95d6bfce58893ecb20590bc0da
// MIT/Apache 2.0 license
// TODO: proper attirubution?
pub trait FallibleCommands {
    /// Despawn entity and all its children.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_despawn_recursive(&mut self, entity: Entity);

    /// Execute command on entity if it exists
    fn try_command<F: FnOnce(&mut EntityWorldMut) + Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        command: F,
    );

    /// Add components to entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_insert(&mut self, entity: Entity, bundle: impl Bundle) {
        self.try_command(entity, move |entity| {
            entity.insert(bundle);
        })
    }

    /// Remove components from entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_remove<B: Bundle>(&mut self, entity: Entity) {
        self.try_command(entity, |entity| {
            entity.remove::<B>();
        })
    }

    /// Spawn children to the entity.
    ///
    /// **Functor execution is delayed, so it can't reference local variables.**
    ///
    /// Silently fails if entity doesn't exist.
    fn try_with_children<F: FnOnce(&mut WorldChildBuilder) + Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        spawn_children: F,
    ) {
        self.try_command(entity, move |entity| {
            entity.with_children(spawn_children);
        })
    }

    /// Spawn bundle as single child to the entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_with_child<B: Bundle>(&mut self, entity: Entity, bundle: B) {
        self.try_command(entity, move |entity| {
            entity.with_children(move |parent| {
                parent.spawn(bundle);
            });
        })
    }
}

impl<'w, 's> FallibleCommands for Commands<'w, 's> {
    fn try_despawn_recursive(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            if let Some(entity_mut) = world.get_entity_mut(entity) {
                entity_mut.despawn_recursive();
            }
        });
    }

    fn try_command<F: FnOnce(&mut EntityWorldMut) + Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        command: F,
    ) {
        if let Some(mut commands) = self.get_entity(entity) {
            commands.add(FallibleEntityCommand(command));
        }
    }
}

struct FallibleEntityCommand<F>(F);

impl<F: FnOnce(&mut EntityWorldMut) + Send + Sync + 'static> EntityCommand
    for FallibleEntityCommand<F>
{
    fn apply(self, id: Entity, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(id) {
            self.0(&mut entity)
        }
    }
}

/// Additional methods for [`EntityWorldMut`]
pub trait ExtendedEntityMut {
    /// Intended for spawning child on entity and getting its id
    fn with_child<F: FnOnce(&mut WorldChildBuilder) -> Entity>(&mut self, builder: F) -> Entity;
}

impl<'w> ExtendedEntityMut for EntityWorldMut<'w> {
    fn with_child<F: FnOnce(&mut WorldChildBuilder) -> Entity>(&mut self, builder: F) -> Entity {
        let mut id = None;
        self.with_children(|parent| id = Some(builder(parent)));
        id.unwrap()
    }
}
