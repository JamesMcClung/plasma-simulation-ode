use super::*;

impl<W: Write> WriteBytes<f64> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &f64) -> Result<usize> {
        match BYTES_PER_WORD {
            4 => self.write(&(*item as f32).to_le_bytes()),
            8 => self.write(&item.to_le_bytes()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f64() {
        let mut data = Vec::new();
        data.write_bytes::<8>(&1.5_f64).unwrap();
        assert_eq!(data, vec![0, 0, 0, 0, 0, 0, 0xf8, 0x3f]);
    }
}
