use super::MechanicSet;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::physics::*;
use crate::gameplay::utils::rotation_from_dir;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

///
#[derive(Component, Clone, Copy)]
pub struct Projectile {
    pub damage: u32,
    pub speed: f32,
    pub size: f32,
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
            Collider::ball(self.size),
            Velocity::linear(direction * self.speed),
            ColliderMassProperties::Mass(5.),
            CollidingEntities::default(),
            ActiveEvents::COLLISION_EVENTS,
            PhysicsType::EnemyProjectile.groups(),
            //
            self,
        )
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            damage: 1,
            speed: 10.,
            size: 0.3,
        }
    }
}

/// When reaches zero, [`Dead`] is added to the entity.
#[derive(Component)]
pub struct Health {
    pub value: u32,
}

impl Health {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

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
    pub damage: u32,
}

///
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileImpact>()
            .add_systems(Update, projectile.in_set(MechanicSet::Reaction))
            .add_systems(PostUpdate, remove_dead_colliders);
    }
}

fn projectile(
    projectiles: Query<(Entity, &Projectile, &CollidingEntities, &GlobalTransform)>,
    mut victims: Query<&mut Health, Without<Dead>>,
    mut commands: Commands,
    mut impacts: EventWriter<ProjectileImpact>,
) {
    for (proj_entity, projectile, colliding, pos) in projectiles.iter() {
        let damage = projectile.damage;

        for victim in colliding.iter() {
            if let Ok(mut health) = victims.get_mut(victim) {
                if health.reduce(damage) {
                    commands.try_insert(victim, Dead);
                }
            }
        }

        if !colliding.is_empty() {
            commands.try_despawn_recursive(proj_entity);
            impacts.send(ProjectileImpact {
                pos: pos.translation().truncate(),
                damage,
            })
        }
    }
}

fn remove_dead_colliders(entity: Query<Entity, Added<Dead>>, mut commands: Commands) {
    for entity in entity.iter() {
        commands.try_remove::<Collider>(entity);
    }
}
