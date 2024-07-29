use std::ops::Div;

use super::*;

impl_vector_binop!(Div, div, /);
impl_scalar_binop_right!(Div, div, /);

impl_scalar_binop_left_for!(Div, div, /, Float);
impl_scalar_binop_left_for!(Div, div, /, Int);
impl_scalar_binop_left_for!(Div, div, /, UInt);
