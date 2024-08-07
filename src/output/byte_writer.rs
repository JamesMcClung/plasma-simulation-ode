mod impl_f32;
mod impl_f64;
mod impl_float_n;
mod impl_i32;
mod impl_i64;
mod impl_particle_species;

use std::io::{Result, Write};
use std::marker::PhantomData;

use super::*;

pub struct ByteWriter<T>(PhantomData<T>);

impl<T> ByteWriter<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
