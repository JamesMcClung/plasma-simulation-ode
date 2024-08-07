use super::*;

impl Writer<i32> for ByteWriter<i32> {
    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[mem::size_of::<i32>() as u8 * 8, Self::TYPE_ID])
    }

    fn write<W: Write>(&self, writer: &mut W, item: &i32) -> Result<usize> {
        writer.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_i32() {
        let mut data = Vec::new();
        let writer = ByteWriter::new();
        writer.write_prelude(&mut data).unwrap();
        writer.write(&mut data, &18_i32).unwrap();
        assert_eq!(data, vec![32, TypeIDs::<Int>::ID, 18, 0, 0, 0]);
    }
}
