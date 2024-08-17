use super::*;

impl<W: Write> WriteBytes<f32> for W {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &f32) -> Result<usize> {
        match BYTES_PER_WORD {
            4 => self.write(&item.to_le_bytes()),
            8 => self.write(&(*item as f64).to_le_bytes()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_f32() {
        let mut data = Vec::new();
        data.write_bytes::<4>(&1.5_f32).unwrap();
        assert_eq!(data, vec![0, 0, 0xc0, 0x3f]);
    }
}
