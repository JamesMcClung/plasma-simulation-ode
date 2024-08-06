use std::io::{Result, Write};

use super::*;

pub struct ByteWriter;

impl Writer<f64> for ByteWriter {
    fn write<W: Write>(writer: &mut W, item: &f64) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }

    fn write_prelude<W: Write>(writer: &mut W) -> Result<usize> {
        writer.write(&[FLOAT_64, ID_FLOAT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        ByteWriter::write_prelude(&mut data).unwrap();
        ByteWriter::write(&mut data, &1.5_f64).unwrap();
        assert_eq!(data, vec![FLOAT_64, ID_FLOAT, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
