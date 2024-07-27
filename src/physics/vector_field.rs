pub mod centering;
mod enumeration;
mod interpolation;
mod location_at;
mod trait_impls;

use centering::Centering;

use crate::prelude::*;

pub struct VectorField<const N_DIMS: usize> {
    centering: Centering<N_DIMS>,
    lower_corner_location: FloatN<N_DIMS>,
    grid_spacing: FloatN<N_DIMS>,
    grid_spacing_inv: FloatN<N_DIMS>,
    dim_lens: UIntN<N_DIMS>,
    n_dims: usize,
    data: [Tensor<N_DIMS>; N_DIMS],
}

impl<const N_DIMS: usize> VectorField<N_DIMS> {
    pub fn new(
        centering: Centering<N_DIMS>,
        dim_lens: impl Into<UIntN<N_DIMS>>,
        corner_location: impl Into<FloatN<N_DIMS>>,
        grid_spacing: impl Into<FloatN<N_DIMS>>, //
    ) -> Self {
        let dim_lens = dim_lens.into();
        let lower_corner_location = corner_location.into();
        let grid_spacing = grid_spacing.into();
        let grid_spacing_inv = 1.0 / grid_spacing;
        let data = std::array::from_fn(|_| Tensor::zeros(dim_lens));

        Self { centering, lower_corner_location, dim_lens, data, grid_spacing, grid_spacing_inv, n_dims: N_DIMS }
    }

    pub fn lower_corner(&self) -> FloatN<N_DIMS> {
        self.lower_corner_location
    }

    pub fn upper_corner(&self) -> FloatN<N_DIMS> {
        self.lower_corner_location + self.grid_spacing * self.dim_lens.map(|val| val as Float)
    }

    pub fn n_dims(&self) -> usize {
        self.n_dims
    }
}
