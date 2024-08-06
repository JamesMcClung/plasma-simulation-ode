mod add_assign;

use super::*;

macro_rules! impl_assign_vector {
    ($trait:ident, $method:ident, $symbol:tt) => {
        impl<T, const LEN: usize> $trait for Vector<T, LEN>
        where
            T: $trait,
        {
            fn $method(&mut self, rhs: Vector<T, LEN>) {
                for (lhs, rhs) in self.iter_mut().zip(rhs.into_iter()) {
                    *lhs $symbol rhs;
                }
            }
        }
    };
}

use impl_assign_vector;
