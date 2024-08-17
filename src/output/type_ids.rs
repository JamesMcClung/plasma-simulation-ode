use crate::prelude::*;

pub trait TypeID {
    const ID: u8;
}

impl TypeID for u8 {
    const ID: u8 = 0;
}

impl TypeID for u32 {
    const ID: u8 = 0;
}

impl TypeID for u64 {
    const ID: u8 = 0;
}

impl TypeID for usize {
    const ID: u8 = 0;
}

impl TypeID for i32 {
    const ID: u8 = 1;
}

impl TypeID for i64 {
    const ID: u8 = 1;
}

impl TypeID for f32 {
    const ID: u8 = 2;
}

impl TypeID for f64 {
    const ID: u8 = 2;
}

impl<T, const LEN: usize> TypeID for Vector<T, LEN> {
    const ID: u8 = 10;
}

impl TypeID for ParticleSpecies {
    const ID: u8 = 20;
}

impl<const N_DIMS: usize> TypeID for ParticleList<N_DIMS> {
    const ID: u8 = 30;
}
