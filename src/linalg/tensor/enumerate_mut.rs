use super::*;

pub struct TensorIterMut<'a, const N_DIMS: usize> {
    data_iter: std::slice::IterMut<'a, Float>,
    dim_lens: UIntN<N_DIMS>,
    next_idx: UIntN<N_DIMS>,
}

impl<const N_DIMS: usize> Tensor<N_DIMS> {
    pub fn enumerate_mut(&mut self) -> TensorIterMut<N_DIMS> {
        let dim_lens = self.dim_lens;
        TensorIterMut { data_iter: self.data.iter_mut(), next_idx: UIntN::zeros(), dim_lens }
    }
}

impl<'a, const N_DIMS: usize> Iterator for TensorIterMut<'a, N_DIMS> {
    type Item = (UIntN<N_DIMS>, &'a mut Float);

    fn next(&mut self) -> Option<Self::Item> {
        let ret_element = self.data_iter.next()?;
        let ret_idx = self.next_idx;

        self.next_idx = Tensor::increment_idx(self.next_idx, self.dim_lens);

        Some((ret_idx, ret_element))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_mut() {
        let dim_lens = [2, 3].into();
        let mut tensor = Tensor::zeros(dim_lens);
        for (idx, val) in tensor.enumerate_mut() {
            *val = Tensor::flatten_idx(idx, dim_lens) as Float;
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
