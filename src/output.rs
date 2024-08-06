use std::io::{Result, Write};

mod byte_writer;

pub const FLOAT_32: u8 = 0;
pub const FLOAT_64: u8 = 1;

pub const ID_FLOAT: u8 = 1;

pub use byte_writer::ByteWriter;

pub trait Writer<T> {
    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize>;
    fn write<W: Write>(&self, writer: &mut W, item: &T) -> Result<usize>;
}
