use std::mem::size_of;

use super::*;
use crate::prelude::*;

impl Writer<ParticleSpecies> for ByteWriter<ParticleSpecies> {
    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[size_of::<Float>() as u8 * 8, TypeIDs::<ParticleSpecies>::ID])
    }

    fn write<W: Write>(&self, writer: &mut W, item: &ParticleSpecies) -> Result<usize> {
        let mut bytes_written = 0;

        let float_writer = ByteWriter::new();
        bytes_written += float_writer.write(writer, &item.mass())?;
        bytes_written += float_writer.write(writer, &item.charge())?;
        bytes_written += float_writer.write(writer, &item.weight())?;

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn write_species_result() -> Vec<u8> {
        vec![
            size_of::<Float>() as u8 * 8, TypeIDs::<ParticleSpecies>::ID, // prelude (float bits, dtype)
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // mass
            0, 0, 0, 0, 0, 0, 0xf8, 0x3f, // charge
            0, 0, 0, 0, 0, 0, 0   , 0x40, // weight
        ]
    }

    #[test]
    fn write_species() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &ParticleSpecies::new(1.0, 1.5, 2.0)).unwrap();
        assert_eq!(data, write_species_result());
    }
}
