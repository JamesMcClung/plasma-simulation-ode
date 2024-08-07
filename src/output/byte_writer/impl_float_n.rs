use super::*;

impl<const LEN: usize> Writer<FloatN<LEN>> for ByteWriter<FloatN<LEN>> {
    fn write<W: Write>(&self, writer: &mut W, item: &FloatN<LEN>) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += writer.write(&[TypeIDs::<Float>::ID, LEN as u8])?;

        let element_writer = ByteWriter::new();
        for el in item.iter() {
            bytes_written += element_writer.write(writer, el)?;
        }

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[rustfmt::skip]
    fn write_float_3_result() -> Vec<u8> {
        vec![
            mem::size_of::<Float>() as u8 * 8, TypeIDs::<Float3>::ID, // prelude (float bits, dtype)
            TypeIDs::<Float>::ID, 3, // vector header (dtype, size)
            0, 0, 0, 0, 0, 0, 0xe0, 0x3f, // first float
            0, 0, 0, 0, 0, 0, 0   , 0   , // second float
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // third float
        ]
    }

    #[test]
    fn write_float_3() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &Float3::from([0.5, 0.0, 1.0])).unwrap();
        assert_eq!(data, write_float_3_result());
    }
}
