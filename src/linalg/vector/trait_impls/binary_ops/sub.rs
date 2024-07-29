use std::ops::Sub;

use super::*;

impl_binops!(Sub, sub, -);

impl_scalar_lop_for!(Sub, sub, -, Float);
impl_scalar_lop_for!(Sub, sub, -, Int);
impl_scalar_lop_for!(Sub, sub, -, UInt);
