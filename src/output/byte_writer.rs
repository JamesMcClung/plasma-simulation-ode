use std::io::{Result, Write};

use super::*;

pub struct ByteWriter;

impl Writer<f64> for ByteWriter {
    fn write<W: Write>(writer: &mut W, item: &f64) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        ByteWriter::write(&mut data, &1.5_f64).unwrap();
        assert_eq!(data, vec![0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
