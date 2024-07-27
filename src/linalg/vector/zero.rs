use super::*;

macro_rules! impl_zero {
    ($type:ident, $zero:literal) => {
        impl<const LEN: usize> Vector<$type, LEN> {
            pub fn zeros() -> Self {
                Self([$zero; LEN])
            }
        }
    };
}

impl_zero!(Float, 0.0);
impl_zero!(Int, 0);
impl_zero!(UInt, 0);
