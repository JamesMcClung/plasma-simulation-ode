use std::ops::Index;

use super::*;

impl<const N_DIMS: usize, Idx> Index<(usize, Idx)> for VectorField<N_DIMS>
where
    Idx: Into<UIntN<N_DIMS>>,
{
    type Output = Float;

    fn index(&self, (component, idx): (usize, Idx)) -> &Self::Output {
        &self.data[component][idx]
    }
}
