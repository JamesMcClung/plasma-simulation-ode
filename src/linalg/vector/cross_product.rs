use std::ops::{Mul, Sub};

use crate::prelude::*;

impl<T> Vector<T, 3>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    pub fn cross(&self, other: Self) -> Self {
        let new_x = self[1] * other[2] - self[2] * other[1];
        let new_y = self[2] * other[0] - self[0] * other[2];
        let new_z = self[0] * other[1] - self[1] * other[0];
        Self([new_x, new_y, new_z])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_vectors() {
        let xhat = IntN::from([1, 0, 0]);
        let yhat = IntN::from([0, 1, 0]);
        let zhat = IntN::from([0, 0, 1]);

        assert_eq!(xhat.cross(yhat), zhat);
        assert_eq!(yhat.cross(zhat), xhat);
        assert_eq!(zhat.cross(xhat), yhat);

        assert_eq!(yhat.cross(xhat), -1 * zhat);
        assert_eq!(zhat.cross(yhat), -1 * xhat);
        assert_eq!(xhat.cross(zhat), -1 * yhat);
    }
}
