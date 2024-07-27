use std::ops::Index;

use super::*;

impl<const N_DIMS: usize, Idx> Index<Idx> for Tensor<N_DIMS>
where
    Idx: Into<UIntN<N_DIMS>>,
{
    type Output = Float;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[self.flatten_idx(index.into())]
    }
}
