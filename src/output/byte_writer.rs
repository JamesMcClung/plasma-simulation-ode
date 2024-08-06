use std::io::{Result, Write};
use std::marker::PhantomData;

use super::*;

pub struct ByteWriter<T>(PhantomData<T>);

impl<T> ByteWriter<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl Writer<f32> for ByteWriter<f32> {
    fn write<W: Write>(&self, writer: &mut W, item: &f32) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }

    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[FLOAT_32, ID_FLOAT])
    }
}

impl Writer<f64> for ByteWriter<f64> {
    fn write<W: Write>(&self, writer: &mut W, item: &f64) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }

    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[FLOAT_64, ID_FLOAT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f32() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &1.5_f32).unwrap();
        assert_eq!(data, vec![FLOAT_32, ID_FLOAT, 0, 0, 0xc0, 0x3f]);
    }

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &1.5_f64).unwrap();
        assert_eq!(data, vec![FLOAT_64, ID_FLOAT, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
