use std::ops::Div;

use super::*;

impl_binops!(Div, div, /);

// can't generically implement left scalar op; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210

macro_rules! impl_left_div {
    ($type:tt) => {
        impl<const LEN: usize, U> Div<Vector<U, LEN>> for $type
        where
            $type: Div<U>,
        {
            type Output = Vector<<$type as Div<U>>::Output, LEN>;

            fn div(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self / rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as Div<U>>::Output, LEN>(res_arr)
            }
        }
    };
}

impl_left_div!(Float);
impl_left_div!(Int);
impl_left_div!(UInt);
