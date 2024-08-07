use super::*;

impl<W: Write> WriteBytes<i64> for W {
    fn write_bytes(&mut self, item: &i64) -> Result<usize> {
        self.write(&item.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_i64() {
        let mut data = Vec::new();
        data.write_prelude::<i64>().unwrap();
        data.write_bytes(&18_i64).unwrap();
        assert_eq!(data, vec![64, TypeIDs::<Int>::ID, 18, 0, 0, 0, 0, 0, 0, 0]);
    }
}
