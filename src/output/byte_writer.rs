mod impl_f32;
mod impl_f64;

use std::io::{Result, Write};
use std::marker::PhantomData;

use super::*;

pub struct ByteWriter<T>(PhantomData<T>);

impl<T> ByteWriter<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
