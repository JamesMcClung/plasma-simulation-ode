use super::*;

impl<T, const LEN: usize> FromIterator<T> for Vector<T, LEN> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self(std::array::from_fn(|_| iter.next().unwrap()))
    }
}
