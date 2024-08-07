use std::marker::PhantomData;

use crate::prelude::*;

pub struct TypeIDs<T>(PhantomData<T>);

pub trait TypeID<T> {
    const ID: u8;
}

impl TypeID<i32> for TypeIDs<i32> {
    const ID: u8 = 0;
}

impl TypeID<i64> for TypeIDs<i64> {
    const ID: u8 = 0;
}

impl TypeID<f32> for TypeIDs<f32> {
    const ID: u8 = 1;
}

impl TypeID<f64> for TypeIDs<f64> {
    const ID: u8 = 1;
}

impl<T, const LEN: usize> TypeID<Vector<T, LEN>> for TypeIDs<Vector<T, LEN>> {
    const ID: u8 = 2;
}

impl TypeID<ParticleSpecies> for TypeIDs<ParticleSpecies> {
    const ID: u8 = 3;
}

impl<const N_DIMS: usize> TypeID<ParticleList<N_DIMS>> for TypeIDs<ParticleList<N_DIMS>> {
    const ID: u8 = 4;
}
