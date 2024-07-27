use super::*;

impl<T, const LEN: usize> Eq for Vector<T, LEN> where T: Eq {}
