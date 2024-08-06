mod boris;

use crate::prelude::*;

pub use boris::BorisStepper;

pub trait ParticleStepper {
    /// Updates particle positions and velocities to/from the following times:
    /// ```txt
    /// velocity(t - delta_t/2) -> velocity(t + delta_t/2)
    /// position(t)             -> position(t + delta_t)
    /// ```
    fn step_particles(
        particles: &mut ParticleList<3>,
        e_field: &VectorField<3>,
        b_field: &VectorField<3>,
        delta_t: Float, //
    );
}
