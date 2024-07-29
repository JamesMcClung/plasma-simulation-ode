use std::ops::Add;

use super::*;

impl_vector_binop!(Add, add, +);
impl_scalar_binop_right!(Add, add, +);

impl_scalar_binop_left_for!(Add, add, +, Float);
impl_scalar_binop_left_for!(Add, add, +, Int);
impl_scalar_binop_left_for!(Add, add, +, UInt);
