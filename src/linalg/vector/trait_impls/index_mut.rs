use super::*;

impl<T, const LEN: usize> std::ops::IndexMut<usize> for Vector<T, LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
