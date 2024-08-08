use std::marker::PhantomData;

pub trait OutputPrimitives {
    type UInt;
    type Int;
    type Float;
    const BYTES_PER_WORD: u8;
}

pub struct MatchPrimitives<T>(PhantomData<T>);

impl OutputPrimitives for MatchPrimitives<f32> {
    type UInt = u32;
    type Int = i32;
    type Float = f32;
    const BYTES_PER_WORD: u8 = 4;
}

impl OutputPrimitives for MatchPrimitives<f64> {
    type UInt = u64;
    type Int = i64;
    type Float = f64;
    const BYTES_PER_WORD: u8 = 8;
}
