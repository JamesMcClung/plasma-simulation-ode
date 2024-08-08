use std::marker::PhantomData;

use crate::prelude::*;

pub struct TypeIDs<T>(PhantomData<T>);

pub trait TypeID {
    const ID: u8;
}

impl TypeID for TypeIDs<u32> {
    const ID: u8 = 0;
}

impl TypeID for TypeIDs<u64> {
    const ID: u8 = 0;
}

impl TypeID for TypeIDs<i32> {
    const ID: u8 = 1;
}

impl TypeID for TypeIDs<i64> {
    const ID: u8 = 1;
}

impl TypeID for TypeIDs<f32> {
    const ID: u8 = 2;
}

impl TypeID for TypeIDs<f64> {
    const ID: u8 = 2;
}

impl<T, const LEN: usize> TypeID for TypeIDs<Vector<T, LEN>> {
    const ID: u8 = 10;
}

impl TypeID for TypeIDs<ParticleSpecies> {
    const ID: u8 = 20;
}

impl<const N_DIMS: usize> TypeID for TypeIDs<ParticleList<N_DIMS>> {
    const ID: u8 = 30;
}
