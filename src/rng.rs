use crate::*;

use bevy::prelude::*;
use chacha20::ChaCha8Rng;

#[derive(Component, Deref, DerefMut)]
pub struct RngSeed(pub u64);

#[derive(Component, Deref, DerefMut)]
pub struct Rng(pub ChaCha8Rng);
