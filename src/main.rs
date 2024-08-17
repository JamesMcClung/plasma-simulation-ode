use pso::prelude::*;
use step_particles::BorisStepper;

fn write_output(step: usize, particles: &ParticleList<3>) {
    FileWriter::<_, 4>::create(format!("runs/run1/{step:03}.particles.dat")).unwrap().write_bytes(particles).unwrap();
}

fn main() {
    let dim_lens = UIntN::from([32, 32, 1]);
    let corner_location = FloatN::zeros();
    let grid_spacing = FloatN::copied(1.0);

    let species = ParticleSpecies::new(1.0, 1.0, 1.0);
    let mut particles = ParticleList::new(species);
    particles.push([1.0, 1.0, 0.0], [0.0, 0.5, 0.0]);

    let e_field = VectorField::new(Centering::NodeCentered, dim_lens, corner_location, grid_spacing);

    let mut b_field = VectorField::new(Centering::NodeCentered, dim_lens, corner_location, grid_spacing);
    for (_pos, val) in b_field.enumerate_space_mut(2) {
        *val = 1.0;
    }

    let delta_t = 1.0 / 32.0;
    let n_steps = 1000;
    let out_interval = 10;

    write_output(0, &particles);
    for step in 1..=n_steps {
        BorisStepper::step_particles(&mut particles, &e_field, &b_field, delta_t);
        if step % out_interval == 0 {
            write_output(step, &particles);
        }
    }
}
