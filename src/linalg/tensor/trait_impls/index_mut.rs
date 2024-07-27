use std::ops::IndexMut;

use super::*;

impl<const N_DIMS: usize, Idx> IndexMut<Idx> for Tensor<N_DIMS>
where
    Idx: Into<UIntN<N_DIMS>>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let index = Tensor::flatten_idx(index.into(), self.dim_lens);
        &mut self.data[index]
    }
}
