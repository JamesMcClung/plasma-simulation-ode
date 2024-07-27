use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Centering<const N_DIMS: usize> {
    NodeCentered,
    EdgeCentered,
    FaceCentered,
    CellCentered,
}

impl<const N_DIMS: usize> Centering<N_DIMS> {
    pub fn get_offset(&self, component: UInt) -> FloatN<N_DIMS> {
        match self {
            Self::NodeCentered => [0.0; N_DIMS].into(),
            Self::EdgeCentered => {
                let mut res = [0.0; N_DIMS];
                res[component] = 0.5;
                res.into()
            },
            Self::FaceCentered => {
                let mut res = [0.5; N_DIMS];
                res[component] = 0.0;
                res.into()
            },
            Self::CellCentered => [0.5; N_DIMS].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_3d() {
        let centering = Centering::<3>::NodeCentered;
        assert_eq!(centering.get_offset(0), [0.0, 0.0, 0.0].into());
        assert_eq!(centering.get_offset(1), [0.0, 0.0, 0.0].into());
        assert_eq!(centering.get_offset(2), [0.0, 0.0, 0.0].into());

        let centering = Centering::<3>::EdgeCentered;
        assert_eq!(centering.get_offset(0), [0.5, 0.0, 0.0].into());
        assert_eq!(centering.get_offset(1), [0.0, 0.5, 0.0].into());
        assert_eq!(centering.get_offset(2), [0.0, 0.0, 0.5].into());

        let centering = Centering::<3>::FaceCentered;
        assert_eq!(centering.get_offset(0), [0.0, 0.5, 0.5].into());
        assert_eq!(centering.get_offset(1), [0.5, 0.0, 0.5].into());
        assert_eq!(centering.get_offset(2), [0.5, 0.5, 0.0].into());

        let centering = Centering::<3>::CellCentered;
        assert_eq!(centering.get_offset(0), [0.5, 0.5, 0.5].into());
        assert_eq!(centering.get_offset(1), [0.5, 0.5, 0.5].into());
        assert_eq!(centering.get_offset(2), [0.5, 0.5, 0.5].into());
    }

    #[test]
    fn offsets_2d() {
        let centering = Centering::<2>::NodeCentered;
        assert_eq!(centering.get_offset(0), [0.0, 0.0].into());
        assert_eq!(centering.get_offset(1), [0.0, 0.0].into());

        let centering = Centering::<2>::EdgeCentered;
        assert_eq!(centering.get_offset(0), [0.5, 0.0].into());
        assert_eq!(centering.get_offset(1), [0.0, 0.5].into());

        let centering = Centering::<2>::FaceCentered;
        assert_eq!(centering.get_offset(0), [0.0, 0.5].into());
        assert_eq!(centering.get_offset(1), [0.5, 0.0].into());

        let centering = Centering::<2>::CellCentered;
        assert_eq!(centering.get_offset(0), [0.5, 0.5].into());
        assert_eq!(centering.get_offset(1), [0.5, 0.5].into());
    }
}
