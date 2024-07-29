mod add;
mod div;
mod mul;
mod sub;

use super::*;

macro_rules! impl_vector_binop {
    ($trait:ident, $method:ident, $symbol:tt) => {
        impl<T, const LEN: usize> $trait for Vector<T, LEN>
        where
            T: $trait,
        {
            type Output = Vector<T::Output, LEN>;

            fn $method(self, rhs: Vector<T, LEN>) -> Self::Output {
                let mut res_iter = self.into_iter().zip(rhs.into_iter()).map(|(lhs, rhs)| lhs $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<T::Output, LEN>(res_arr)
            }
        }
    };
}

macro_rules! impl_scalar_binop_right {
    ($trait:ident, $method:ident, $symbol:tt) => {
        impl<T, const LEN: usize> $trait<T> for Vector<T, LEN>
        where
            T: $trait + Copy,
        {
            type Output = Vector<T::Output, LEN>;

            fn $method(self, rhs: T) -> Self::Output {
                let mut res_iter = self.into_iter().map(|lhs| lhs $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<T::Output, LEN>(res_arr)
            }
        }
    };
}

// can't generically implement scalar left-operation; see
// https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210
macro_rules! impl_scalar_binop_left_for {
    ($trait:ident, $method:ident, $symbol:tt, $type:tt) => {
        impl<const LEN: usize> $trait<Vector<$type, LEN>> for $type
        {
            type Output = Vector<<$type as $trait>::Output, LEN>;

            fn $method(self, rhs: Vector<$type, LEN>) -> Self::Output {
                let mut res_iter = rhs.into_iter().map(|rhs| self $symbol rhs);
                let res_arr = std::array::from_fn(|_| res_iter.next().unwrap());
                Vector::<<$type as $trait>::Output, LEN>(res_arr)
            }
        }
    };
}

use impl_scalar_binop_left_for;
use impl_scalar_binop_right;
use impl_vector_binop;
