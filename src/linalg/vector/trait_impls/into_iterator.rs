use super::*;

impl<T, const LEN: usize> IntoIterator for Vector<T, LEN> {
    type IntoIter = std::array::IntoIter<T, LEN>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
