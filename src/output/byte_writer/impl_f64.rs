use std::mem::size_of;

use super::*;

impl Writer<f64> for ByteWriter<f64> {
    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[size_of::<f64>() as u8 * 8, ID_FLOAT])
    }

    fn write<W: Write>(&self, writer: &mut W, item: &f64) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &1.5_f64).unwrap();
        assert_eq!(data, vec![64, ID_FLOAT, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
