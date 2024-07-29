use std::ops::Sub;

use super::*;

impl_vector_binop!(Sub, sub, -);
impl_scalar_binop_right!(Sub, sub, -);

impl_scalar_binop_left_for!(Sub, sub, -, Float);
impl_scalar_binop_left_for!(Sub, sub, -, Int);
impl_scalar_binop_left_for!(Sub, sub, -, UInt);
