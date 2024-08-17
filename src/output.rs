mod file_writer;
mod impls;
mod type_ids;

use std::io::{Result, Write};

use crate::prelude::*;
use type_ids::TypeID;

pub use file_writer::FileWriter;

const FORMAT_VERSION_MAJOR: u8 = 0;
const FORMAT_VERSION_MINOR: u8 = 1;
const FORMAT_VERSION_PATCH: u8 = 0;

pub trait WriteBytes<T: TypeID>: Write {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &T) -> Result<usize>;
}
