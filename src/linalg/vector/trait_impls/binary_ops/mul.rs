use std::ops::Mul;

use super::*;

impl_binops!(Mul, mul, *);

// can't generically implement left scalar op; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210

macro_rules! impl_left_mul {
    ($type:tt) => {
        impl<const LEN: usize, U> Mul<Vector<U, LEN>> for $type
        where
            $type: Mul<U>,
        {
            type Output = Vector<<$type as Mul<U>>::Output, LEN>;

            fn mul(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self * rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as Mul<U>>::Output, LEN>(res_arr)
            }
        }
    };
}

impl_left_mul!(Float);
impl_left_mul!(Int);
impl_left_mul!(UInt);
