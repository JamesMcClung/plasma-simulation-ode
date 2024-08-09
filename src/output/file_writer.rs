use std::{
    fs::File,
    io::{BufWriter, Result},
    path::PathBuf,
};

use super::*;

pub struct FileWriter<const BYTES_PER_WORD: u8 = { crate::output::BYTES_PER_WORD }> {
    file: BufWriter<File>,
}

impl<const BYTES_PER_WORD: u8> FileWriter<BYTES_PER_WORD> {
    /// Opens a file and immediately writes the [PRELUDE].
    pub fn create(path: impl Into<PathBuf>) -> Result<Self> {
        let mut file = BufWriter::new(File::create(path.into())?);
        file.write(&PRELUDE)?;
        Ok(Self { file })
    }

    pub fn write_bytes<T: TypeID>(&mut self, item: &T) -> Result<usize>
    where
        BufWriter<File>: WriteBytes<T>,
    {
        self.file.write_bytes::<BYTES_PER_WORD>(item)
    }
}
