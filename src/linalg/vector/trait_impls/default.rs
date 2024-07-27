use super::*;

impl<T, const LEN: usize> Default for Vector<T, LEN>
where
    T: Default,
{
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}
