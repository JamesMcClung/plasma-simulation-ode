use super::*;

impl<W: Write> WriteBytes<i64> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &i64) -> Result<usize> {
        match BYTES_PER_WORD {
            4 => self.write(&(*item as i32).to_le_bytes()),
            8 => self.write(&item.to_le_bytes()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_i64() {
        let mut data = Vec::new();
        data.write_bytes::<8>(&18_i64).unwrap();
        assert_eq!(data, vec![18, 0, 0, 0, 0, 0, 0, 0]);
    }
}
