use super::*;

/// A stepper implementing the leapfrog method for position and velocity, using the Boris method for velocity.
///
/// The Boris method is an implicit method for the Lorentz equation that avoids matrix inversion.
/// Starting with the implicit equation (where `x` is the cross product):
/// ```txt
/// (v[n+1/2] - v[n-1/2]) / dt = q/m * (E + (v[n+1/2] + v[n-1/2]) / 2 x B),
/// ```
/// we define `v[+]` and `v[-]` via
/// ```txt
/// v[n-1/2] = v[-] - q/m * E * dt/2
/// v[n+1/2] = v[+] + q/m * E * dt/2
/// ```
/// to obtain
/// ```txt
/// (v[+] - v[-]) / dt = q/m * (v[+] + v[-]) x B / 2.
/// ```
/// We have now a half-step using only `E` to go from `v[n-1/2]` to `v[-]`, a rotation from `v[-]` to `v[+]` using only `B`, and another half-step using only `E` to go from `v[+]` to `v[n+1/2]`.
///
/// See [BorisStepper::step_velocities_b] for implementation details of the rotation.
pub struct BorisStepper;

impl BorisStepper {
    fn step_positions(
        positions: &mut Vec<FloatN<3>>,
        velocities: &Vec<FloatN<3>>,
        delta_t: Float, //
    ) {
        for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
            *pos += (*vel) * delta_t;
        }
    }

    fn step_velocities_half_e(
        positions: &Vec<FloatN<3>>,
        velocities: &mut Vec<FloatN<3>>,
        species: ParticleSpecies,
        e_field: &VectorField<3>,
        half_delta_t: Float, //
    ) {
        for (vel, pos) in velocities.iter_mut().zip(positions.iter()) {
            *vel += species.charge_mass_ratio() * e_field.interpolate(*pos) * half_delta_t;
        }
    }

    /// Performs the pure rotation from `v[-]` to `v[+]` See [BorisStepper] for definitions, and note `•` is the dot product.
    ///
    /// The angle `theta` through which to rotate is given by
    /// ```txt
    /// theta = -q/m * |B| * dt,
    /// ```
    /// so for small `dt`, we can approximate
    /// ```txt
    /// tan(theta/2) ~ theta/2 = -q/m * |B| * dt/2.
    /// ```
    /// The axis of rotation is, of course, B, so the vector representing the rotation is given by
    /// ```txt
    /// t = -B/|B| * tan(theta/2)
    ///   = q/m * B * dt/2.
    /// ```
    /// The vector bisecting the angle between `v[-]` and `v[+]` is given by
    /// ```txt
    /// v' = v[-] + v[-] x t.
    /// ```
    /// Note `v'` is in the `v[-]`-`v[+]` plane, since `t` *defines* the plane. Additionally, the angle between `v'` and `v[-]` is `theta/2`, since
    /// ```txt
    /// v' • v[-] / (|v'| * |v[-]|) = (v[-]^2 + 0) / (sqrt(v[-]^2 + (v[-] x t)^2) * |v[-]|)
    ///                             = v[-]^2 / (sqrt(v[-]^2 + (v[-] x t)^2) * |v[-]|)
    ///                             = |v[-]| / sqrt(v[-]^2 + v[-]^2 * t^2)
    ///                             = 1 / sqrt(1 + t^2)
    ///                             = 1 / sqrt(1 + tan^2(theta/2))
    ///                             = cos(theta/2) / sqrt(cos^2(theta/2) + sin^2(theta/2))
    ///                             = cos(theta/2).
    /// ```
    /// Finally, we rotate again using a properly scaled copy of `t`:
    /// ```txt
    /// s = t * 2/(1 + t^2)
    /// v[+] = v[-] + v' x s.
    /// ```
    /// This works because
    /// ```txt
    /// v[+] • v[-] = (v[-] + v' x s) • v[-]
    ///             = v[-]^2 + (v' x s) • v[-]
    ///             = v[-]^2 + (v[-] x v') • s
    ///             = v[-]^2 + (v[-] x (v[-] + v[-] x t)) • s
    ///             = v[-]^2 + (v[-] x (v[-] x t)) • s
    ///             = v[-]^2 + (v[-] • t * v[-] - v[-] • v[-] * t) • s
    ///             = v[-]^2 + (0 - v[-]^2 * t) • s
    ///             = v[-]^2 - v[-]^2 * t • s
    ///             = v[-]^2 * (1 - t • s)
    ///             = v[-]^2 * (1 - t^2 * 2/(1 + t^2))
    ///             = v[-]^2 * (1 - t^2) / (1 + t^2)
    ///             = v[-]^2 * (1 - tan^2(theta/2)) / (1 + tan^2(theta/2))
    ///             = v[-]^2 * (cos^2(theta/2) - sin^2(theta/2)) / (cos^2(theta/2) + sin^2(theta/2))
    ///             = v[-]^2 * cos(theta).
    /// ```
    fn step_velocities_b(
        positions: &Vec<FloatN<3>>,
        velocities: &mut Vec<FloatN<3>>,
        species: ParticleSpecies,
        b_field: &VectorField<3>,
        half_delta_t: Float, //
    ) {
        for (vel, pos) in velocities.iter_mut().zip(positions.iter()) {
            let t = species.charge_mass_ratio() * b_field.interpolate(*pos) * half_delta_t;
            let v_prime = *vel + vel.cross(t);
            let s = 2.0 * t / (1.0 + t.mag2());
            *vel += v_prime.cross(s);
        }
    }
}

impl ParticleStepper for BorisStepper {
    fn step_particles(
        particles: &mut ParticleList<3>,
        e_field: &VectorField<3>,
        b_field: &VectorField<3>,
        delta_t: Float, //
    ) {
        let half_delta_t = 0.5 * delta_t;
        Self::step_velocities_half_e(&particles.positions, &mut particles.velocities, particles.species, e_field, half_delta_t);
        Self::step_velocities_b(&particles.positions, &mut particles.velocities, particles.species, b_field, half_delta_t);
        Self::step_velocities_half_e(&particles.positions, &mut particles.velocities, particles.species, e_field, half_delta_t);
        Self::step_positions(&mut particles.positions, &particles.velocities, delta_t);
    }
}
