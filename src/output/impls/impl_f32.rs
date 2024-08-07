use super::*;

impl<W: Write> WriteBytes<f32> for W {
    fn write_bytes(&mut self, item: &f32) -> Result<usize> {
        self.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f32() {
        let mut data = Vec::new();
        data.write_prelude::<f32>().unwrap();
        data.write_bytes(&1.5_f32).unwrap();
        assert_eq!(data, vec![64, TypeIDs::<Float>::ID, 0, 0, 0xc0, 0x3f]);
    }
}
