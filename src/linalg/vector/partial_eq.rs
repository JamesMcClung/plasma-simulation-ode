use super::*;

impl<T, const LEN: usize> PartialEq for Vector<T, LEN>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}
