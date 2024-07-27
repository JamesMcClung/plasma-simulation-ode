use itertools::Itertools;

use super::*;

impl<const N_DIMS: usize> std::fmt::Debug for Tensor<N_DIMS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter_all: Box<dyn Iterator<Item = UIntN<N_DIMS>>> = Box::new(std::iter::once(UIntN::zeros()));
        for dim_idx in 0..N_DIMS {
            iter_all = Box::new(
                iter_all
                    .cartesian_product(0..self.dim_len(dim_idx)) //
                    .map(move |(mut prev_idxs, new_idx)| {
                        prev_idxs[dim_idx] = new_idx;
                        prev_idxs
                    }),
            )
        }

        let n_elements = self.dim_lens.iter().product();
        for (i, idx) in iter_all.enumerate() {
            self[idx].fmt(f)?;

            if i + 1 != n_elements {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
}
