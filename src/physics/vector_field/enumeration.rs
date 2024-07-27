use super::*;

impl<const N_DIMS: usize> VectorField<N_DIMS> {
    pub fn enumerate_grid(&self, component_idx: usize) -> impl Iterator<Item = (UIntN<N_DIMS>, &Float)> {
        self.data[component_idx].enumerate()
    }

    pub fn enumerate_grid_mut(&mut self, component_idx: usize) -> impl Iterator<Item = (UIntN<N_DIMS>, &mut Float)> {
        self.data[component_idx].enumerate_mut()
    }

    pub fn enumerate_space(&self, component_idx: usize) -> impl Iterator<Item = (FloatN<N_DIMS>, &Float)> {
        self.data[component_idx].enumerate().map(move |(grid_idx, val)| (self.location_at(component_idx, grid_idx), val))
    }

    pub fn enumerate_space_mut(&mut self, component_idx: usize) -> impl Iterator<Item = (FloatN<N_DIMS>, &mut Float)> {
        let lower_corner_location = self.lower_corner_location;
        let grid_spacing = self.grid_spacing;
        let centering = self.centering;
        self.data[component_idx].enumerate_mut().map(move |(grid_idx, val)| {
            (
                Self::location_at_impl(
                    component_idx, //
                    grid_idx,
                    lower_corner_location,
                    grid_spacing,
                    centering,
                ),
                val,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_space() {
        let field = VectorField::new(Centering::EdgeCentered, [2, 2, 2], FloatN::zeros(), FloatN::copied(1.0));

        let mut field_iter = field.enumerate_space(0);
        assert_eq!(field_iter.next(), Some(([0.5, 0.0, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.5, 0.0, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.5, 1.0, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.5, 1.0, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.5, 0.0, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.5, 0.0, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.5, 1.0, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.5, 1.0, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), None);

        let mut field_iter = field.enumerate_space(1);
        assert_eq!(field_iter.next(), Some(([0.0, 0.5, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 0.5, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 1.5, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 1.5, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 0.5, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 0.5, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 1.5, 0.0].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 1.5, 1.0].into(), &0.0)));
        assert_eq!(field_iter.next(), None);

        let mut field_iter = field.enumerate_space(2);
        assert_eq!(field_iter.next(), Some(([0.0, 0.0, 0.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 0.0, 1.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 1.0, 0.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([0.0, 1.0, 1.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 0.0, 0.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 0.0, 1.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 1.0, 0.5].into(), &0.0)));
        assert_eq!(field_iter.next(), Some(([1.0, 1.0, 1.5].into(), &0.0)));
        assert_eq!(field_iter.next(), None);
    }
}
