use std::ops::Add;

use super::*;

impl_binops!(Add, add, +);

impl_scalar_lop_for!(Add, add, +, Float);
impl_scalar_lop_for!(Add, add, +, Int);
impl_scalar_lop_for!(Add, add, +, UInt);
