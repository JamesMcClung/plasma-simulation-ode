use super::*;

impl<T, const LEN: usize> From<[T; LEN]> for Vector<T, LEN> {
    fn from(value: [T; LEN]) -> Self {
        Self(value)
    }
}
