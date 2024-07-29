mod trait_impls;
mod zero;

use crate::prelude::*;

pub struct Vector<T, const LEN: usize>([T; LEN]);

pub type UIntN<const LEN: usize> = Vector<UInt, LEN>;
pub type IntN<const LEN: usize> = Vector<Int, LEN>;
pub type FloatN<const LEN: usize> = Vector<Float, LEN>;

pub type UInt2 = UIntN<2>;
pub type Int2 = IntN<2>;
pub type Float3 = FloatN<3>;

impl<T, const LEN: usize> Vector<T, LEN> {
    pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
        Self(std::array::from_fn(f))
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.0.iter_mut()
    }

    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Vector<U, LEN> {
        self.into_iter().map(f).collect()
    }
}
