use crate::prelude::*;

pub struct ParticleList<const N_DIMS: usize> {
    pub positions: Vec<FloatN<N_DIMS>>,
    pub velocities: Vec<FloatN<N_DIMS>>,
    mass: Float,
    charge: Float,
    weight: Float,
    charge_mass_ratio: Float,
}

impl<const N_DIMS: usize> ParticleList<N_DIMS> {
    pub fn new(mass: Float, charge: Float, weight: Float) -> Self {
        Self { positions: Vec::new(), velocities: Vec::new(), mass, charge, weight, charge_mass_ratio: charge / mass }
    }

    pub fn mass(&self) -> Float {
        self.mass
    }

    pub fn charge(&self) -> Float {
        self.charge
    }

    pub fn weight(&self) -> Float {
        self.weight
    }

    pub fn charge_mass_ratio(&self) -> Float {
        self.charge_mass_ratio
    }
}
