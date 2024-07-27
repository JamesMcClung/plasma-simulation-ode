use super::*;

pub struct TensorIter<'a, const N_DIMS: usize> {
    tensor: &'a Tensor<N_DIMS>,
    next_idx_flat: usize,
    next_idx: UIntN<N_DIMS>,
}

impl<const N_DIMS: usize> Tensor<N_DIMS> {
    pub fn enumerate<'a>(&'a self) -> TensorIter<'a, N_DIMS> {
        TensorIter { tensor: self, next_idx_flat: 0, next_idx: UIntN::zeros() }
    }
}

impl<'a, const N_DIMS: usize> Iterator for TensorIter<'a, N_DIMS> {
    type Item = (UIntN<N_DIMS>, &'a Float);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_idx_flat >= self.tensor.data.len() {
            return None;
        }

        let ret_element = &self.tensor.data[self.next_idx_flat];
        let ret_idx = self.next_idx;

        self.next_idx_flat += 1;
        self.next_idx = Tensor::increment_idx(self.next_idx, self.tensor.dim_lens);

        Some((ret_idx, ret_element))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate() {
        let mut tensor = Tensor::zeros([2, 3]);
        for (flat_idx, val) in tensor.data.iter_mut().enumerate() {
            *val = flat_idx as Float;
        }

        let mut enumerator = tensor.enumerate();

        assert_eq!(enumerator.next(), Some(([0, 0].into(), &0.0)));
        assert_eq!(enumerator.next(), Some(([0, 1].into(), &1.0)));
        assert_eq!(enumerator.next(), Some(([0, 2].into(), &2.0)));
        assert_eq!(enumerator.next(), Some(([1, 0].into(), &3.0)));
        assert_eq!(enumerator.next(), Some(([1, 1].into(), &4.0)));
        assert_eq!(enumerator.next(), Some(([1, 2].into(), &5.0)));
        assert_eq!(enumerator.next(), None);
    }
}
