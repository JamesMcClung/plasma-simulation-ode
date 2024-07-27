use std::ops::IndexMut;

use super::*;

impl<const N_DIMS: usize, Idx> IndexMut<Idx> for Tensor<N_DIMS>
where
    Idx: Into<UIntN<N_DIMS>>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let index = self.flatten_idx(index.into());
        &mut self.data[index]
    }
}
