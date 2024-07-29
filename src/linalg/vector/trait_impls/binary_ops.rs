mod add;
mod div;
mod mul;
mod sub;

use super::*;

macro_rules! impl_binops {
    ($trait:ident, $method:ident, $symbol:tt) => {
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
    };
}

use impl_binops;
