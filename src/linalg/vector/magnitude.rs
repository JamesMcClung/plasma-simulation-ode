use std::{iter::Sum, ops::Mul};

use crate::prelude::*;

impl<T, const LEN: usize> Vector<T, LEN>
where
    T: Mul<Output = T> + Copy + Sum,
{
    pub fn mag2(&self) -> T {
        self.iter().map(|el| *el * *el).sum()
    }
}
