use super::*;

impl<W: Write> WriteBytes<ParticleSpecies> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &ParticleSpecies) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += self.write_bytes::<BYTES_PER_WORD>(&item.mass())?;
        bytes_written += self.write_bytes::<BYTES_PER_WORD>(&item.charge())?;
        bytes_written += self.write_bytes::<BYTES_PER_WORD>(&item.weight())?;

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn write_species_result() -> Vec<u8> {
        vec![
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // mass
            0, 0, 0, 0, 0, 0, 0xf8, 0x3f, // charge
            0, 0, 0, 0, 0, 0, 0   , 0x40, // weight
        ]
    }

    #[test]
    fn write_species() {
        let mut data = Vec::new();
        data.write_bytes::<8>(&ParticleSpecies::new(1.0, 1.5, 2.0)).unwrap();
        assert_eq!(data, write_species_result());
    }
}
