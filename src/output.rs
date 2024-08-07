mod byte_writer;
mod type_ids;

use std::io::{Result, Write};
use std::mem;

use crate::prelude::*;

pub use byte_writer::ByteWriter;
use type_ids::{TypeID, TypeIDs};

pub trait Writer<T>
where
    TypeIDs<T>: TypeID<T>,
{
    const TYPE_ID: u8 = TypeIDs::<T>::ID;

    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&[mem::size_of::<Float>() as u8 * 8, Self::TYPE_ID])
    }

    fn write<W: Write>(&self, writer: &mut W, item: &T) -> Result<usize>;
}
