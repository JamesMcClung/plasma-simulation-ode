use super::*;

impl<T, const LEN: usize> Copy for Vector<T, LEN> where T: Copy {}

impl<T, const LEN: usize> Vector<T, LEN>
where
    T: Copy,
{
    pub fn copied(val: T) -> Self {
        Self([val; LEN])
    }
}
