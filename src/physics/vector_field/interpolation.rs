use super::*;
use crate::linalg::neighbors_iter::NeighborsIter;

impl<const N_DIMS: usize> VectorField<N_DIMS> {
    fn interpolate_component_impl(
        &self,
        component_idx: UInt,
        location: FloatN<N_DIMS>, //
    ) -> Float {
        let idx = (location - self.lower_corner_location) * self.grid_spacing_inv - self.centering.get_offset(component_idx);
        for i in idx {
            // casting a negative float to an unsized int silently yields 0,
            // so we have to manually check
            assert!(i >= 0.0);
        }
        let idx_fract = idx.map(Float::fract);
        let idx = idx.map(|i| i.trunc() as UInt);

        let field_component = &self.data[component_idx];

        let axis_weights: [[Float; 2]; N_DIMS] = std::array::from_fn(|i| [1.0 - idx_fract[i], idx_fract[i]]);

        let mut res = 0.0;
        for neighbor in NeighborsIter::new() {
            let weight: Float = axis_weights.iter().zip(neighbor.iter()).map(|(comp_weights, comp_idx)| comp_weights[*comp_idx]).product();
            if weight != 0.0 {
                res += weight * field_component[idx + neighbor];
            }
        }

        res
    }

    pub fn interpolate_component(&self, component_idx: UInt, location: impl Into<FloatN<N_DIMS>>) -> Float {
        let location = location.into();
        self.interpolate_component_impl(component_idx, location)
    }

    pub fn interpolate(&self, location: impl Into<FloatN<N_DIMS>>) -> FloatN<N_DIMS> {
        let location = location.into();
        FloatN::from_fn(|i| self.interpolate_component(i, location))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_1d() {
        let dim_lens = [4];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0], [1.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([1.0]), [1.0].into());
        assert_eq!(field.interpolate([1.25]), [1.25].into());
        assert_eq!(field.interpolate([1.5]), [1.5].into());
        assert_eq!(field.interpolate([1.75]), [1.75].into());
        assert_eq!(field.interpolate([2.0]), [2.0].into());
    }

    #[test]
    fn interpolate_edge() {
        let dim_lens = [3];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0], [1.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([0.0]), [0.0].into());
        assert_eq!(field.interpolate([2.0]), [2.0].into());
    }

    #[should_panic]
    #[test]
    fn interpolate_out_of_bounds_below() {
        let dim_lens = [4];
        let field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0], [1.0]);

        dbg!(field.interpolate([-0.5]));
    }

    #[should_panic]
    #[test]
    fn interpolate_out_of_bounds_above() {
        let dim_lens = [4];
        let field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0], [1.0]);

        field.interpolate([3.5]);
    }

    #[test]
    fn interpolate_2d() {
        let dim_lens = [2, 4];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0, 0.0], [1.0, 1.0]);
        for i in 0..dim_lens[0] {
            for j in 0..dim_lens[1] {
                field[(0, [i, j])] = (i * dim_lens[1] + j) as Float;
                field[(1, [i, j])] = (j * dim_lens[0] + i) as Float;
            }
        }

        // component 0 should be:
        // 0 1 2 3
        // 4 5 6 7
        // component 1 should be:
        // 0 2 4 6
        // 1 3 5 7

        // across edge 1
        assert_eq!(field.interpolate([0.0, 1.0]), [1.0, 2.0].into());
        assert_eq!(field.interpolate([0.25, 1.0]), [2.0, 2.25].into());
        assert_eq!(field.interpolate([0.5, 1.0]), [3.0, 2.5].into());
        assert_eq!(field.interpolate([1.0, 1.0]), [5.0, 3.0].into());

        // across edge 2
        assert_eq!(field.interpolate([0.0, 1.0]), [1.0, 2.0].into());
        assert_eq!(field.interpolate([0.0, 1.25]), [1.25, 2.5].into());
        assert_eq!(field.interpolate([0.0, 1.5]), [1.5, 3.0].into());
        assert_eq!(field.interpolate([0.0, 2.0]), [2.0, 4.0].into());

        // across center
        assert_eq!(field.interpolate([0.0, 1.0]), [1.0, 2.0].into());
        assert_eq!(field.interpolate([0.25, 1.25]), [2.25, 2.75].into());
        assert_eq!(field.interpolate([0.5, 1.5]), [3.5, 3.5].into());
        assert_eq!(field.interpolate([1.0, 2.0]), [6.0, 5.0].into());
    }

    #[test]
    fn interpolate_shifted_corner() {
        let dim_lens = [4];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [5.0], [1.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([5.0]), [0.0].into());
        assert_eq!(field.interpolate([6.5]), [1.5].into());
    }

    #[test]
    fn interpolate_shifted_corner_negative() {
        let dim_lens = [4];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [-1.0], [1.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([-1.0]), [0.0].into());
        assert_eq!(field.interpolate([-0.75]), [0.25].into());
        assert_eq!(field.interpolate([-0.5]), [0.5].into());
        assert_eq!(field.interpolate([-0.25]), [0.75].into());
        assert_eq!(field.interpolate([0.0]), [1.0].into());
        assert_eq!(field.interpolate([0.25]), [1.25].into());
    }

    #[test]
    fn interpolate_nonunit_spacing() {
        let dim_lens = [4];
        let mut field = VectorField::new(Centering::NodeCentered, dim_lens, [0.0], [2.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([1.0]), [0.5].into());
        assert_eq!(field.interpolate([2.0]), [1.0].into());
        assert_eq!(field.interpolate([2.5]), [1.25].into());
    }

    #[test]
    fn interpolate_cc() {
        let dim_lens = [4];
        let mut field = VectorField::new(Centering::CellCentered, dim_lens, [0.0], [1.0]);
        for i in 0..dim_lens[0] {
            field[(0, [i])] = i as Float;
        }

        assert_eq!(field.interpolate([1.0]), [0.5].into());
        assert_eq!(field.interpolate([1.25]), [0.75].into());
        assert_eq!(field.interpolate([1.5]), [1.0].into());
    }
}
