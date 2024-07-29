use std::ops::Div;

use super::*;

impl_binops!(Div, div, /);

impl_scalar_lop_for!(Div, div, /, Float);
impl_scalar_lop_for!(Div, div, /, Int);
impl_scalar_lop_for!(Div, div, /, UInt);
