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

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn push(&mut self, position: impl Into<FloatN<N_DIMS>>, velocity: impl Into<FloatN<N_DIMS>>) {
        self.positions.push(position.into());
        self.velocities.push(velocity.into());
    }

    pub fn kinetic_energy(&self) -> Float {
        0.5 * self.species.mass() * self.velocities.iter().map(Vector::mag2).sum::<Float>() * self.species.weight()
    }
}
