use crate::prelude::*;

pub struct NeighborsIter<const N_DIMS: usize> {
    flat_idx: usize,
    end_flat_idx: usize,
}

impl<const N_DIMS: usize> NeighborsIter<N_DIMS> {
    pub fn new() -> Self {
        Self { flat_idx: 0, end_flat_idx: 1 << N_DIMS }
    }
}

impl<const N_DIMS: usize> Iterator for NeighborsIter<N_DIMS> {
    type Item = UIntN<N_DIMS>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flat_idx == self.end_flat_idx {
            return None;
        }

        let mut res = UIntN::zeros();
        for i in 0..N_DIMS {
            if self.flat_idx & (1 << i) > 0 {
                res[N_DIMS - 1 - i] = 1;
            }
        }

        self.flat_idx += 1;

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_neighbors_3d() {
        let mut iter = NeighborsIter::<3>::new();

        assert_eq!(iter.next(), Some([0, 0, 0].into()));
        assert_eq!(iter.next(), Some([0, 0, 1].into()));
        assert_eq!(iter.next(), Some([0, 1, 0].into()));
        assert_eq!(iter.next(), Some([0, 1, 1].into()));
        assert_eq!(iter.next(), Some([1, 0, 0].into()));
        assert_eq!(iter.next(), Some([1, 0, 1].into()));
        assert_eq!(iter.next(), Some([1, 1, 0].into()));
        assert_eq!(iter.next(), Some([1, 1, 1].into()));
        assert_eq!(iter.next(), None);
    }
}
