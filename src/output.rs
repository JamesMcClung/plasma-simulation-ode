use std::io::{Result, Write};

mod byte_writer;

pub trait Writer<T> {
    fn write<W: Write>(writer: &mut W, item: &T) -> Result<usize>;
}
