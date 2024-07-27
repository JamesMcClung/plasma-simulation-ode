use super::*;

impl<T, const LEN: usize> Clone for Vector<T, LEN>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const LEN: usize> Vector<T, LEN>
where
    T: Clone,
{
    pub fn cloned(val: &T) -> Self {
        Self::from_fn(|_| val.clone())
    }
}
