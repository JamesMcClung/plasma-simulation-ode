mod byte_writer;
mod type_ids;

use std::io::{Result, Write};
use std::mem;

use crate::prelude::*;

use type_ids::{TypeID, TypeIDs};

pub trait WriteBytes<T>: Write
where
    TypeIDs<T>: TypeID,
{
    fn write_bytes(&mut self, item: &T) -> Result<usize>;
}

pub trait WritePrelude: Write {
    fn write_prelude<T>(&mut self) -> Result<usize>
    where
        TypeIDs<T>: TypeID,
    {
        self.write(&[mem::size_of::<Float>() as u8 * 8, TypeIDs::<T>::ID])
    }
}

impl<W: Write> WritePrelude for W {}
