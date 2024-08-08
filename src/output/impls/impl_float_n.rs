use super::*;

impl<const LEN: usize, W: Write> WriteBytes<FloatN<LEN>> for W {
    fn write_bytes(&mut self, item: &FloatN<LEN>) -> Result<usize> {
        let mut bytes_written = 0;

        bytes_written += self.write(&[Float::ID, LEN as u8])?;

        for el in item.iter() {
            bytes_written += self.write_bytes(el)?;
        }

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn write_float_3_result() -> Vec<u8> {
        vec![
            Float::ID, 3,                 // vector header (dtype, size)
            0, 0, 0, 0, 0, 0, 0xe0, 0x3f, // first float
            0, 0, 0, 0, 0, 0, 0   , 0   , // second float
            0, 0, 0, 0, 0, 0, 0xf0, 0x3f, // third float
        ]
    }

    #[test]
    fn write_float_3() {
        let mut data = Vec::new();
        data.write_bytes(&Float3::from([0.5, 0.0, 1.0])).unwrap();
        assert_eq!(data, write_float_3_result());
    }
}
