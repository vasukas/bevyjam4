use super::MechanicSet;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::physics::*;
use crate::gameplay::utils::rotation_from_dir;
use crate::gameplay::utils::Lifetime;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use std::time::Duration;

///
#[derive(Component, Clone, Copy)]
pub struct Projectile {
    pub damage: u32,
    pub speed: f32,
    pub radius: f32,
    pub ty: DamageType,
}

impl Projectile {
    /// Game object bundle for moving projectile
    pub fn bundle(self, position: Vec2, direction: Vec2) -> impl Bundle {
        (
            GameObjectBundle::new(
                "projectile",
                Transform::from_translation(position.extend(0.))
                    .with_rotation(rotation_from_dir(direction)),
            ),
            //
            RigidBody::Dynamic,
            Collider::ball(self.radius),
            Velocity::linear(direction * self.speed),
            ColliderMassProperties::Mass(20.),
            CollidingEntities::default(),
            ActiveEvents::COLLISION_EVENTS,
            PhysicsType::Projectile.groups(),
            //
            Lifetime(Duration::from_millis(5000)),
            self,
        )
    }
}

/// Damage is applied only if damage type and health type match
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Player,
    Barrels,
}

/// When reaches zero, [`Dead`] is added to the entity.
#[derive(Component)]
pub struct Health {
    pub value: u32,
    pub ty: DamageType,
}

impl Health {
    fn reduce(&mut self, by: u32) -> bool {
        self.value = self.value.saturating_sub(by);
        self.value == 0
    }
}

/// Added when health (or its equivavient) reaches zero.
///
/// Colliders are removed on death.
#[derive(Component)]
pub struct Dead;

///
#[derive(Event, Clone, Copy)]
pub struct ProjectileImpact {
    pub pos: Vec2,
    pub projectile: Projectile,
}

/// Send this to apply damage to an entity
#[derive(Event)]
pub struct ApplyDamage {
    pub victim: Entity,
    pub amount: u32,
    pub ty: DamageType,
}

///
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileImpact>()
            .add_event::<ApplyDamage>()
            .add_systems(
                Update,
                (projectile, apply_damage)
                    .chain()
                    .in_set(MechanicSet::Reaction),
            )
            .add_systems(PostUpdate, remove_dead_colliders);
    }
}

fn projectile(
    projectiles: Query<(Entity, &Projectile, &CollidingEntities, &GlobalTransform)>,
    mut commands: Commands,
    mut impacts: EventWriter<ProjectileImpact>,
    mut apply_damage: EventWriter<ApplyDamage>,
) {
    for (proj_entity, projectile, colliding, pos) in projectiles.iter() {
        for victim in colliding.iter() {
            apply_damage.send(ApplyDamage {
                victim,
                amount: projectile.damage,
                ty: projectile.ty,
            })
        }

        if !colliding.is_empty() {
            commands.try_despawn_recursive(proj_entity);
            impacts.send(ProjectileImpact {
                pos: pos.translation().truncate(),
                projectile: *projectile,
            })
        }
    }
}

fn apply_damage(
    mut damage: EventReader<ApplyDamage>,
    mut victims: Query<&mut Health, Without<Dead>>,
    mut commands: Commands,
) {
    for damage in damage.read() {
        if let Ok(mut health) = victims.get_mut(damage.victim) {
            if health.ty == damage.ty && health.reduce(damage.amount) {
                commands.try_insert(damage.victim, Dead);
            }
        }
    }
}

fn remove_dead_colliders(entity: Query<Entity, Added<Dead>>, mut commands: Commands) {
    for entity in entity.iter() {
        commands.try_remove::<RigidBody>(entity);
        commands.try_remove::<Collider>(entity);
    }
}
