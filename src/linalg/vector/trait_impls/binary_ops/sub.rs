use std::ops::Sub;

use super::*;

impl_binops!(Sub, sub, -);

// can't generically implement left scalar op; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210

macro_rules! impl_left_sub {
    ($type:tt) => {
        impl<const LEN: usize, U> Sub<Vector<U, LEN>> for $type
        where
            $type: Sub<U>,
        {
            type Output = Vector<<$type as Sub<U>>::Output, LEN>;

            fn sub(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self - rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as Sub<U>>::Output, LEN>(res_arr)
            }
        }
    };
}

impl_left_sub!(Float);
impl_left_sub!(Int);
impl_left_sub!(UInt);
