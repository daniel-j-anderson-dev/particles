use crate::{AppConfig, Particle, ParticleCount, ParticleKind, ParticleKindCount};

use bevy::prelude::*;
use chacha20::ChaCha8Rng;
use rand::RngExt;

#[derive(Component, Deref, DerefMut)]
pub struct RngSeed(pub u64);

#[derive(Component, Deref, DerefMut)]
pub struct Rng(pub ChaCha8Rng);

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
            // Pre-calculate the material/color for each particle kind so they are split evenly
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
        for particle in particles {
            commands.spawn(particle);
        }
    }
}
