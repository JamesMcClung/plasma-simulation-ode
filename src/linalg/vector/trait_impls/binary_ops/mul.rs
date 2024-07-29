use std::ops::Mul;

use super::*;

impl_vector_binop!(Mul, mul, *);
impl_scalar_binop_right!(Mul, mul, *);

impl_scalar_binop_left_for!(Mul, mul, *, Float);
impl_scalar_binop_left_for!(Mul, mul, *, Int);
impl_scalar_binop_left_for!(Mul, mul, *, UInt);
