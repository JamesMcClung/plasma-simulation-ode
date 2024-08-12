use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::PathBuf,
};

use super::*;

pub struct FileWriter<F: Write = File, const BYTES_PER_WORD: u8 = { std::mem::size_of::<Float>() as u8 }> {
    file: BufWriter<F>,
}

impl<F: Write, const BYTES_PER_WORD: u8> FileWriter<F, BYTES_PER_WORD> {
    fn write_prelude(&mut self) -> Result<usize> {
        self.file.write(&[FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FORMAT_VERSION_PATCH, BYTES_PER_WORD])
    }

    pub fn write<T: TypeID>(&mut self, item: &T) -> Result<usize>
    where
        BufWriter<F>: WriteBytes<T>,
    {
        self.write_type_id::<T>()?;
        self.write_bytes(item)
    }

    pub fn write_type_id<T: TypeID>(&mut self) -> Result<usize> {
        self.file.write_bytes::<BYTES_PER_WORD>(&T::ID)
    }

    pub fn write_bytes<T: TypeID>(&mut self, item: &T) -> Result<usize>
    where
        BufWriter<F>: WriteBytes<T>,
    {
        self.file.write_bytes::<BYTES_PER_WORD>(item)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

impl<const BYTES_PER_WORD: u8> FileWriter<File, BYTES_PER_WORD> {
    /// Opens a (buffered) file and immediately writes the prelude.
    pub fn create(path: impl Into<PathBuf>) -> Result<Self> {
        let file = BufWriter::new(File::create(path.into())?);
        let mut res = Self { file };
        res.write_prelude()?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_prelude() {
        let file = BufWriter::new(Vec::new());
        let mut writer: FileWriter<_, 4> = FileWriter { file };
        writer.write_prelude().unwrap();
        writer.flush().unwrap();
        assert_eq!(writer.file.get_ref(), &[FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FORMAT_VERSION_PATCH, 4]);
    }

    #[rustfmt::skip]
    #[test]
    fn write_stuff_4_bytes() {
        let file = BufWriter::new(Vec::new());
        let mut writer: FileWriter<_, 4> = FileWriter { file };
        writer.write(&1.25_f64).unwrap();
        writer.write(&1.25_f32).unwrap();
        writer.write(&12_u32).unwrap();
        writer.flush().unwrap();
        assert_eq!(writer.file.get_ref(), &[
            f64::ID, 0, 0   , 0   , // id of first float
            0      , 0, 0xa0, 0x3f, // value of first float
            f32::ID, 0, 0   , 0   , // id of second float
            0      , 0, 0xa0, 0x3f, // value of second float
            u32::ID, 0, 0   , 0   , // id of uint
            0x0c   , 0, 0   , 0   , // value of uint
        ]);
    }
}
