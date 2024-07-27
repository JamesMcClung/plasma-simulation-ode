mod enumerate;
mod trait_impls;

use crate::prelude::*;

pub struct Tensor<const N_DIMS: usize> {
    dim_lens: UIntN<N_DIMS>,
    data: Vec<Float>,
}

impl<const N_DIMS: usize> Tensor<N_DIMS> {
    pub fn zeros(dim_lens: impl Into<UIntN<N_DIMS>>) -> Self {
        let dim_lens = dim_lens.into();
        let n_elements = dim_lens.iter().product();
        let data = vec![0.0; n_elements];
        Self { dim_lens, data }
    }

    pub fn identity(dim_lens: UInt) -> Self {
        let mut res = Self::zeros([dim_lens; N_DIMS]);
        for i in 0..dim_lens {
            res[[i; N_DIMS]] = 1.0;
        }
        res
    }

    pub fn dim_len(&self, dim_idx: usize) -> UInt {
        self.dim_lens[dim_idx]
    }

    fn flatten_idx(idx: UIntN<N_DIMS>, dim_lens: UIntN<N_DIMS>) -> UInt {
        for i in 0..N_DIMS {
            assert!(idx[i] < dim_lens[i]);
        }

        let mut flat_idx = idx[0];
        for i in 1..N_DIMS {
            flat_idx *= dim_lens[i];
            flat_idx += idx[i];
        }

        flat_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let dims = [3, 4];
        let zeros = Tensor::<2>::zeros(dims);

        for i in 0..dims[0] {
            for j in 0..dims[1] {
                assert_eq!(zeros[[i, j]], 0.0);
            }
        }
    }

    #[test]
    fn identity() {
        let dim = 3;
        let identity = Tensor::identity(dim);

        for i in 0..dim {
            for j in 0..dim {
                let element = if i == j { 1.0 } else { 0.0 };
                assert_eq!(identity[[i, j]], element);
            }
        }
    }

    #[test]
    fn index_mut() {
        let dims = [2, 5];
        let mut mat = Tensor::zeros(dims);

        let idx = [1, 4];
        let val = 3.0;
        mat[idx] = val;
        assert_eq!(mat[idx], val);
    }

    #[should_panic]
    #[test]
    fn index_out_of_bounds() {
        let dims = [1, 4];
        let mat = Tensor::zeros(dims);

        let bad_idx = [1, 1];
        mat[bad_idx];
    }

    #[should_panic]
    #[test]
    fn index_mut_out_of_bounds() {
        let dims = [1, 4];
        let mut mat = Tensor::zeros(dims);

        let bad_idx = [1, 1];
        mat[bad_idx] = 2.0;
    }
}
