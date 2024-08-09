mod impls;
mod primitives;
mod type_ids;

use std::io::{Result, Write};

use crate::prelude::*;

use primitives::{MatchPrimitives, OutputPrimitives};
use type_ids::TypeID;

const FORMAT_VERSION_MAJOR: u8 = 0;
const FORMAT_VERSION_MINOR: u8 = 1;
const FORMAT_VERSION_PATCH: u8 = 0;

type OutputUInt = <MatchPrimitives<Float> as OutputPrimitives>::UInt;
type OutputInt = <MatchPrimitives<Float> as OutputPrimitives>::Int;
type OutputFloat = <MatchPrimitives<Float> as OutputPrimitives>::Float;
const BYTES_PER_WORD: u8 = <MatchPrimitives<Float> as OutputPrimitives>::BYTES_PER_WORD;

const PRELUDE: [u8; 4] = [FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FORMAT_VERSION_PATCH, BYTES_PER_WORD];

pub trait WriteBytes<T: TypeID>: Write {
    fn write_bytes<const BYTES_PER_WORD: u8>(&mut self, item: &T) -> Result<usize>;
}
