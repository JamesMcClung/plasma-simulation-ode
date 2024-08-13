pub mod centering;
mod enumeration;
mod interpolation;
mod location_at;
mod trait_impls;

use crate::prelude::*;

pub use centering::Centering;

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

    /// The "highest" node-centered spatial coordinate within the bounds of this field's domain. Certain components may be higher if a centering other than node-centered is used.
    /// ```
    /// # use pso::prelude::*;
    /// let lower_corner = [0.0, 0.0];
    /// let dim_lens = [4, 1];
    /// let grid_spacing = [1.0, 1.0];
    /// let field = VectorField::new(Centering::NodeCentered, dim_lens, lower_corner, grid_spacing);
    /// assert_eq!(field.upper_corner(), [3.0, 0.0].into());
    /// ```
    pub fn upper_corner(&self) -> FloatN<N_DIMS> {
        self.lower_corner_location + self.grid_spacing * self.dim_lens.map(|val| (val - 1) as Float)
    }

    pub fn n_dims(&self) -> usize {
        self.n_dims
    }
}
