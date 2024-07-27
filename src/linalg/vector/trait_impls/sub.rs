use std::ops::Sub;

use super::*;

impl<T, const LEN: usize, U> Sub<Vector<U, LEN>> for Vector<T, LEN>
where
    T: Sub<U>,
{
    type Output = Vector<T::Output, LEN>;

    fn sub(self, rhs: Vector<U, LEN>) -> Self::Output {
        let mut res_iter = self.into_iter().zip(rhs.into_iter()).map(|(lhs, rhs)| lhs - rhs);
        let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
        Vector::<T::Output, LEN>(res_arr)
    }
}

impl<T, const LEN: usize, U> Sub<U> for Vector<T, LEN>
where
    T: Sub<U>,
    U: Copy + NotVector,
{
    type Output = Vector<T::Output, LEN>;

    fn sub(self, rhs: U) -> Self::Output {
        let mut res_iter = self.into_iter().map(|lhs| lhs - rhs);
        let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
        Vector::<T::Output, LEN>(res_arr)
    }
}

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
