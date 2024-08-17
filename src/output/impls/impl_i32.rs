use super::*;

impl<W: Write> WriteBytes<i32> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &i32) -> Result<usize> {
        match BYTES_PER_WORD {
            4 => self.write(&item.to_le_bytes()),
            8 => self.write(&(*item as i64).to_le_bytes()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_i32() {
        let mut data = Vec::new();
        data.write_bytes::<4>(&18_i32).unwrap();
        assert_eq!(data, vec![18, 0, 0, 0]);
    }
}
