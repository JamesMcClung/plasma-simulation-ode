use std::ops::IndexMut;

use super::*;

impl<const N_DIMS: usize, Idx> IndexMut<(usize, Idx)> for VectorField<N_DIMS>
where
    Idx: Into<UIntN<N_DIMS>>,
{
    fn index_mut(&mut self, (component, idx): (usize, Idx)) -> &mut Self::Output {
        &mut self.data[component][idx]
    }
}
