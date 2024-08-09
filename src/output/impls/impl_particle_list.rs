use super::*;

impl<W: Write, const N_DIMS: usize> WriteBytes<ParticleList<N_DIMS>> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &ParticleList<N_DIMS>) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += self.write(&[N_DIMS as u8])?;
        bytes_written += self.write_bytes::<BYTES_PER_WORD>(&item.species)?;
        bytes_written += self.write_bytes::<BYTES_PER_WORD>(&(item.len() as Int))?;

        for pos in item.positions.iter() {
            for val in pos.iter() {
                bytes_written += self.write_bytes::<BYTES_PER_WORD>(val)?;
            }
        }
        for vel in item.velocities.iter() {
            for val in vel.iter() {
                bytes_written += self.write_bytes::<BYTES_PER_WORD>(val)?;
            }
        }

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn write_particle_list_2_result() -> Vec<u8> {
        vec![
            2,                            // number of dimensions
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // species mass
            0, 0, 0, 0, 0, 0, 0xf8, 0x3f, // species charge
            0, 0, 0, 0, 0, 0, 0   , 0x40, // species weight
            2, 0, 0, 0, 0, 0, 0   , 0   , // number of particles
            0, 0, 0, 0, 0, 0, 0   , 0   , // position 1 component 0
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // position 1 component 1
            0, 0, 0, 0, 0, 0, 0   , 0x40, // position 2 component 0
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // position 2 component 1
            0, 0, 0, 0, 0, 0, 0xe0, 0x3f, // velocity 1 component 0
            0, 0, 0, 0, 0, 0, 0   , 0   , // velocity 1 component 1
            0, 0, 0, 0, 0, 0, 0xf8, 0x3f, // velocity 2 component 0
            0, 0, 0, 0, 0, 0, 0   , 0   , // velocity 2 component 1
        ]
    }

    #[test]
    fn write_particle_list_2() {
        let species = ParticleSpecies::new(1.0, 1.5, 2.0);
        let mut particles = ParticleList::new(species);
        particles.push([0.0, 1.0], [0.5, 0.0]);
        particles.push([2.0, 1.0], [1.5, 0.0]);

        let mut data = Vec::new();
        data.write_bytes::<8>(&particles).unwrap();
        assert_eq!(data, write_particle_list_2_result());
    }
}
