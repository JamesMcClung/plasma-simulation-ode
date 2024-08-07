use super::*;

impl Writer<i64> for ByteWriter<i64> {
    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[mem::size_of::<i64>() as u8 * 8, Self::TYPE_ID])
    }

    fn write<W: Write>(&self, writer: &mut W, item: &i64) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_i64() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &18_i64).unwrap();
        assert_eq!(data, vec![64, TypeIDs::<Int>::ID, 18, 0, 0, 0, 0, 0, 0, 0]);
    }
}
