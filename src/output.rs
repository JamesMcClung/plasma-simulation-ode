mod impls;
mod primitives;
mod type_ids;

use std::io::{Result, Write};
use std::mem;

use crate::prelude::*;

use primitives::{MatchPrimitives, OutputPrimitives};
use type_ids::{TypeID, TypeIDs};

const FORMAT_VERSION_MAJOR: u8 = 0;
const FORMAT_VERSION_MINOR: u8 = 1;
const FORMAT_VERSION_PATCH: u8 = 0;

type OutputUInt = <MatchPrimitives<Float> as OutputPrimitives>::UInt;
type OutputInt = <MatchPrimitives<Float> as OutputPrimitives>::Int;
type OutputFloat = <MatchPrimitives<Float> as OutputPrimitives>::Float;
const BYTES_PER_WORD: u8 = <MatchPrimitives<Float> as OutputPrimitives>::BYTES_PER_WORD;

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
