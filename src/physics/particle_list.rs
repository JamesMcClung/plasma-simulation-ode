mod species;

use crate::prelude::*;

pub use species::ParticleSpecies;

pub struct ParticleList<const N_DIMS: usize> {
    pub positions: Vec<FloatN<N_DIMS>>,
    pub velocities: Vec<FloatN<N_DIMS>>,
    pub species: ParticleSpecies,
}

impl<const N_DIMS: usize> ParticleList<N_DIMS> {
    pub fn new(species: ParticleSpecies) -> Self {
        Self { positions: Vec::new(), velocities: Vec::new(), species }
    }
}
