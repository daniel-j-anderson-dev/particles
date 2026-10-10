use crate::*;

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

pub fn spawn_random_particles(
    mut commands: Commands,
    app_config: Query<(Entity, &ParticleKindCount, &ParticleCount, &mut Rng), With<AppConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let random_particle_groups = app_config.into_iter().map(
        |(
            app_config,
            &ParticleKindCount(particle_kind_count),
            &ParticleCount(particle_count),
            mut rng,
        )| {
            let kind_materials: Vec<_> = (0..particle_kind_count)
                .map(|kind| {
                    let hue = (360.0 / particle_kind_count as f32) * kind as f32;
                    materials.add(Color::hsv(hue, 0.95, 0.7))
                })
                .collect();

            let circle = meshes.add(Circle::new(4.0)); // Shared mesh or vary per particle if needed

            let particles = (0..particle_count).map(move |i| {
                // Distribute particle kinds evenly across the total particle count
                let kind = i % particle_kind_count;
                let color = kind_materials[kind].clone();
                let circle = circle.clone();
                let position = vec3(
                    rng.random_range(-400.0..=400.0),
                    rng.random_range(-300.0..=300.0),
                    0.0,
                );

                (
                    Particle,
                    ParticleKind(kind),
                    Transform::from_translation(position),
                    Mesh2d(circle),
                    MeshMaterial2d(color),
                )
            });
            (app_config, particles)
        },
    );

    for (_app_config, particles) in random_particle_groups {
        for p in particles {
            commands.spawn(p);
        }
    }
}

pub fn apply_particle_behavior(
    particle_positions: Query<&mut Transform, With<Particle>>,
    particle_behavior: Single<&ParticleBehavior>,
) {
}
