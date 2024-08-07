use super::*;

impl<W: Write> WriteBytes<ParticleSpecies> for W {
    fn write_bytes(&mut self, item: &ParticleSpecies) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += self.write_bytes(&item.mass())?;
        bytes_written += self.write_bytes(&item.charge())?;
        bytes_written += self.write_bytes(&item.weight())?;

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[rustfmt::skip]
    fn write_species_result() -> Vec<u8> {
        vec![
            mem::size_of::<Float>() as u8 * 8, TypeIDs::<ParticleSpecies>::ID, // prelude (float bits, dtype)
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // mass
            0, 0, 0, 0, 0, 0, 0xf8, 0x3f, // charge
            0, 0, 0, 0, 0, 0, 0   , 0x40, // weight
        ]
    }

    #[test]
    fn write_species() {
        let mut data = Vec::new();
        data.write_prelude::<ParticleSpecies>().unwrap();
        data.write_bytes(&ParticleSpecies::new(1.0, 1.5, 2.0)).unwrap();
        assert_eq!(data, write_species_result());
    }
}
