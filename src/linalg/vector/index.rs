use super::*;

impl<T, const LEN: usize> std::ops::Index<usize> for Vector<T, LEN> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
