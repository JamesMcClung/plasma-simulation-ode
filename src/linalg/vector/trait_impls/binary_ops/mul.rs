use std::ops::Mul;

use super::*;

impl_binops!(Mul, mul, *);

impl_scalar_lop_for!(Mul, mul, *, Float);
impl_scalar_lop_for!(Mul, mul, *, Int);
impl_scalar_lop_for!(Mul, mul, *, UInt);
