use particles::ParticlesPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ParticlesPlugin))
        .run();
}
