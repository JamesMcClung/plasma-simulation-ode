use super::*;

impl<const N_DIMS: usize> Writer<ParticleList<N_DIMS>> for ByteWriter<ParticleList<N_DIMS>> {
    fn write<W: Write>(&self, writer: &mut W, item: &ParticleList<N_DIMS>) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += writer.write(&[N_DIMS as u8])?;
        bytes_written += ByteWriter::new().write(writer, &item.species)?;
        bytes_written += ByteWriter::new().write(writer, &(item.len() as Int))?;

        let float_writer = ByteWriter::new();
        for pos in item.positions.iter() {
            for val in pos.iter() {
                bytes_written += float_writer.write(writer, val)?;
            }
        }
        for vel in item.velocities.iter() {
            for val in vel.iter() {
                bytes_written += float_writer.write(writer, val)?;
            }
        }

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[rustfmt::skip]
    fn write_particle_list_2_result() -> Vec<u8> {
        vec![
            mem::size_of::<Float>() as u8 * 8, TypeIDs::<ParticleList<2>>::ID, // prelude (float bits, dtype)
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
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &particles).unwrap();
        assert_eq!(data, write_particle_list_2_result());
    }
}
