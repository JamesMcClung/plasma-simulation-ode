mod add;
mod div;
mod mul;
mod sub;

use super::*;

macro_rules! impl_binops {
    ($trait:ident, $method:ident, $symbol:tt) => {
        // vector operation
        impl<T, const LEN: usize, U> $trait<Vector<U, LEN>> for Vector<T, LEN>
        where
            T: $trait<U>,
        {
            type Output = Vector<T::Output, LEN>;

            fn $method(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = self.into_iter().zip(rhs.into_iter()).map(|(lhs, rhs)| lhs $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<T::Output, LEN>(res_arr)
            }
        }

        // scalar right-operation
        impl<T, const LEN: usize, U> $trait<U> for Vector<T, LEN>
        where
            T: $trait<U>,
            U: Copy + NotVector,
        {
            type Output = Vector<T::Output, LEN>;

            fn $method(self, rhs: U) -> Self::Output {
                let mut res_iter = self.into_iter().map(|lhs| lhs $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<T::Output, LEN>(res_arr)
            }
        }
    };
}

// can't generically implement scalar left-operation; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210
macro_rules! impl_scalar_lop_for {
    // scalar left-operation
    ($trait:ident, $method:ident, $symbol:tt, $type:tt) => {
        impl<const LEN: usize, U> $trait<Vector<U, LEN>> for $type
        where
            $type: $trait<U>,
        {
            type Output = Vector<<$type as $trait<U>>::Output, LEN>;

            fn $method(self, rhs: Vector<U, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as $trait<U>>::Output, LEN>(res_arr)
            }
        }
    };
}

use impl_binops;
use impl_scalar_lop_for;
