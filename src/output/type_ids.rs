use std::marker::PhantomData;

use crate::prelude::*;

pub struct TypeIDs<T>(PhantomData<T>);

pub trait TypeID<T> {
    const ID: u8;
}

impl TypeID<Float> for TypeIDs<Float> {
    const ID: u8 = 1;
}

impl<T, const LEN: usize> TypeID<Vector<T, LEN>> for TypeIDs<Vector<T, LEN>> {
    const ID: u8 = 2;
}
