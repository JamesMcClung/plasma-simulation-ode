use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct ParticleSpecies {
    mass: Float,
    charge: Float,
    weight: Float,
    charge_mass_ratio: Float,
}

impl ParticleSpecies {
    pub fn new(mass: Float, charge: Float, weight: Float) -> Self {
        Self { mass, charge, weight, charge_mass_ratio: charge / mass }
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
