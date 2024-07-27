use super::*;

impl<const N_DIMS: usize> VectorField<N_DIMS> {
    pub(super) fn location_at_impl(
        component_idx: usize, //
        grid_idx: UIntN<N_DIMS>,
        lower_corner_location: FloatN<N_DIMS>,
        grid_spacing: FloatN<N_DIMS>,
        centering: Centering<N_DIMS>,
    ) -> FloatN<N_DIMS> {
        lower_corner_location + grid_spacing * (grid_idx.map(|val| val as Float) + centering.get_offset(component_idx))
    }

    pub fn location_at(&self, component_idx: usize, grid_idx: impl Into<UIntN<N_DIMS>>) -> FloatN<N_DIMS> {
        Self::location_at_impl(component_idx, grid_idx.into(), self.lower_corner_location, self.grid_spacing, self.centering)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_at_nc() {
        let dim_lens = [3, 3];
        let lower_corner_location = [1.0, -1.0];
        let grid_spacing = [1.0, 2.0];
        let field = VectorField::new(Centering::NodeCentered, dim_lens, lower_corner_location, grid_spacing);

        assert_eq!(field.location_at(0, [0, 0]), [1.0, -1.0].into());
        assert_eq!(field.location_at(0, [1, 0]), [2.0, -1.0].into());
        assert_eq!(field.location_at(0, [0, 1]), [1.0, 1.0].into());
        assert_eq!(field.location_at(0, [1, 1]), [2.0, 1.0].into());
        assert_eq!(field.location_at(0, [2, 2]), [3.0, 3.0].into());

        assert_eq!(field.location_at(1, [0, 0]), [1.0, -1.0].into());
        assert_eq!(field.location_at(1, [1, 0]), [2.0, -1.0].into());
        assert_eq!(field.location_at(1, [0, 1]), [1.0, 1.0].into());
        assert_eq!(field.location_at(1, [1, 1]), [2.0, 1.0].into());
        assert_eq!(field.location_at(1, [2, 2]), [3.0, 3.0].into());
    }

    #[test]
    fn location_at_cc() {
        let dim_lens = [3, 3];
        let lower_corner_location = [1.0, -1.0];
        let grid_spacing = [1.0, 2.0];
        let field = VectorField::new(Centering::CellCentered, dim_lens, lower_corner_location, grid_spacing);

        assert_eq!(field.location_at(0, [0, 0]), [1.5, 0.0].into());
        assert_eq!(field.location_at(0, [1, 0]), [2.5, 0.0].into());
        assert_eq!(field.location_at(0, [0, 1]), [1.5, 2.0].into());
        assert_eq!(field.location_at(0, [1, 1]), [2.5, 2.0].into());
        assert_eq!(field.location_at(0, [2, 2]), [3.5, 4.0].into());

        assert_eq!(field.location_at(1, [0, 0]), [1.5, 0.0].into());
        assert_eq!(field.location_at(1, [1, 0]), [2.5, 0.0].into());
        assert_eq!(field.location_at(1, [0, 1]), [1.5, 2.0].into());
        assert_eq!(field.location_at(1, [1, 1]), [2.5, 2.0].into());
        assert_eq!(field.location_at(1, [2, 2]), [3.5, 4.0].into());
    }

    #[test]
    fn location_at_ec() {
        let dim_lens = [3, 3];
        let lower_corner_location = [1.0, -1.0];
        let grid_spacing = [1.0, 2.0];
        let field = VectorField::new(Centering::EdgeCentered, dim_lens, lower_corner_location, grid_spacing);

        assert_eq!(field.location_at(0, [0, 0]), [1.5, -1.0].into());
        assert_eq!(field.location_at(0, [1, 0]), [2.5, -1.0].into());
        assert_eq!(field.location_at(0, [0, 1]), [1.5, 1.0].into());
        assert_eq!(field.location_at(0, [1, 1]), [2.5, 1.0].into());
        assert_eq!(field.location_at(0, [2, 2]), [3.5, 3.0].into());

        assert_eq!(field.location_at(1, [0, 0]), [1.0, 0.0].into());
        assert_eq!(field.location_at(1, [1, 0]), [2.0, 0.0].into());
        assert_eq!(field.location_at(1, [0, 1]), [1.0, 2.0].into());
        assert_eq!(field.location_at(1, [1, 1]), [2.0, 2.0].into());
        assert_eq!(field.location_at(1, [2, 2]), [3.0, 4.0].into());
    }

    #[test]
    fn location_at_fc() {
        let dim_lens = [3, 3];
        let lower_corner_location = [1.0, -1.0];
        let grid_spacing = [1.0, 2.0];
        let field = VectorField::new(Centering::FaceCentered, dim_lens, lower_corner_location, grid_spacing);

        assert_eq!(field.location_at(0, [0, 0]), [1.0, 0.0].into());
        assert_eq!(field.location_at(0, [1, 0]), [2.0, 0.0].into());
        assert_eq!(field.location_at(0, [0, 1]), [1.0, 2.0].into());
        assert_eq!(field.location_at(0, [1, 1]), [2.0, 2.0].into());
        assert_eq!(field.location_at(0, [2, 2]), [3.0, 4.0].into());

        assert_eq!(field.location_at(1, [0, 0]), [1.5, -1.0].into());
        assert_eq!(field.location_at(1, [1, 0]), [2.5, -1.0].into());
        assert_eq!(field.location_at(1, [0, 1]), [1.5, 1.0].into());
        assert_eq!(field.location_at(1, [1, 1]), [2.5, 1.0].into());
        assert_eq!(field.location_at(1, [2, 2]), [3.5, 3.0].into());
    }
}
