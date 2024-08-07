use super::*;

impl<W: Write> WriteBytes<f64> for W {
    fn write_bytes(&mut self, item: &f64) -> Result<usize> {
        self.write(&(*item as Float).to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        data.write_prelude::<Float>().unwrap();
        data.write_bytes(&1.5_f64).unwrap();
        assert_eq!(data, vec![64, TypeIDs::<Float>::ID, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
