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
