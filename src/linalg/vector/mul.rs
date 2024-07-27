use std::ops::Mul;

use super::*;

impl<T, const LEN: usize, U> Mul<Vector<U, LEN>> for Vector<T, LEN>
where
    T: Mul<U>,
{
    type Output = Vector<T::Output, LEN>;

    fn mul(self, rhs: Vector<U, LEN>) -> Self::Output {
        let mut res_iter = self.into_iter().zip(rhs.into_iter()).map(|(lhs, rhs)| lhs * rhs);
        let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
        Vector::<T::Output, LEN>(res_arr)
    }
}

impl<T, const LEN: usize, U> Mul<U> for Vector<T, LEN>
where
    T: Mul<U>,
    U: Copy + NotVector,
{
    type Output = Vector<T::Output, LEN>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut res_iter = self.into_iter().map(|lhs| lhs * rhs);
        let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
        Vector::<T::Output, LEN>(res_arr)
    }
}

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
