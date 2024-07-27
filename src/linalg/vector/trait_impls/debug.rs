use std::fmt::Debug;

use super::*;

impl<T, const LEN: usize> Debug for Vector<T, LEN>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, element) in self.iter().enumerate() {
            element.fmt(f)?;
            if i + 1 < LEN {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")
    }
}
