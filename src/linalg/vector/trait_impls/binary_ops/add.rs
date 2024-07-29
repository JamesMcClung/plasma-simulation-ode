use std::ops::Add;

use super::*;

impl_binops!(Add, add, +);

impl<T, const LEN: usize, U> Add<U> for Vector<T, LEN>
where
    T: Add<U>,
    U: Copy + NotVector,
{
    type Output = Vector<T::Output, LEN>;

    fn add(self, rhs: U) -> Self::Output {
        let mut res_iter = self.into_iter().map(|lhs| lhs + rhs);
        let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
        Vector::<T::Output, LEN>(res_arr)
    }
}

// can't generically implement left scalar op; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210

macro_rules! impl_left_add {
    ($type:tt) => {
        impl<const LEN: usize, U> Add<Vector<U, LEN>> for $type
        where
            $type: Add<U>,
        {
            type Output = Vector<<$type as Add<U>>::Output, LEN>;

            fn add(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self + rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as Add<U>>::Output, LEN>(res_arr)
            }
        }
    };
}

impl_left_add!(Float);
impl_left_add!(Int);
impl_left_add!(UInt);
