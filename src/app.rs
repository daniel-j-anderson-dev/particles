use crate::*;

use bevy::prelude::*;
use chacha20::ChaCha8Rng;
use rand::SeedableRng;

pub struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                PreStartup,
                (
                    spawn_default_app_config, //
                ),
            )
            .add_systems(
                Startup,
                (
                    spawn_camera, //
                    spawn_random_particle_behavior,
                    spawn_random_particles,
                ),
            )
            .add_systems(
                Update,
                (
                    apply_particle_behavior, //
                ),
            );
    }
}

#[derive(Component)]
pub struct AppConfig;

pub fn spawn_default_app_config(mut commands: Commands) {
    commands.spawn((
        AppConfig,
        ParticleKindCount(4),
        ParticleBehaviorWeightRange {
            start: -1.0,
            end: 1.0,
        },
        Rng(ChaCha8Rng::seed_from_u64(42)),
        ParticleCount(500),
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
