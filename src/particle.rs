use crate::{AppConfig, Rng};

use bevy::prelude::*;
use rand::RngExt;

#[derive(Component, Deref, DerefMut)]
pub struct ParticleKindCount(pub usize);

#[derive(Component, Deref, DerefMut)]
pub struct ParticleKind(pub usize);

#[derive(Component, Deref, DerefMut)]
pub struct ParticleCount(pub usize);

#[derive(Component)]
pub struct ParticleBehavior {
    /// row major adjacency matrix
    weights: Vec<f64>,
    /// width and height of [ParticleBehavior::weights]
    particle_kind_count: usize,
}

#[derive(Component)]
pub struct ParticleBehaviorWeightRange {
    pub start: f64,
    pub end: f64,
}

#[derive(Component)]
pub struct Particle;

pub fn spawn_random_particle_behavior(
    mut commands: Commands,
    query: Query<
        (
            &ParticleKindCount, //
            &ParticleBehaviorWeightRange,
            &mut Rng,
        ),
        With<AppConfig>,
    >,
) {
    let particle_behaviors = query.into_iter().map(
        |(
            &ParticleKindCount(particle_kind_count),
            &ParticleBehaviorWeightRange { start, end },
            mut rng,
        )| {
            let connection_count = particle_kind_count * particle_kind_count;
            let random_weight = || rng.random_range(start..=end);
            ParticleBehavior {
                weights: core::iter::repeat_with(random_weight)
                    .take(connection_count)
                    .collect(),
                particle_kind_count,
            }
        },
    );
    for particle_behavior in particle_behaviors {
        commands.spawn(particle_behavior);
    }
}

pub fn apply_particle_behavior(
    particle_positions: Query<&mut Transform, With<Particle>>,
    particle_behavior: Single<&ParticleBehavior>,
) {
}
