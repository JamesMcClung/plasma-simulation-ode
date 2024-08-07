mod byte_writer;
mod type_ids;

use std::io::{Result, Write};

pub use byte_writer::ByteWriter;
use type_ids::{TypeID, TypeIDs};

pub trait Writer<T>
where
    TypeIDs<T>: TypeID<T>,
{
    const TYPE_ID: u8 = TypeIDs::<T>::ID;

    fn write_prelude<W: Write>(&self, writer: &mut W) -> Result<usize>;
    fn write<W: Write>(&self, writer: &mut W, item: &T) -> Result<usize>;
}
