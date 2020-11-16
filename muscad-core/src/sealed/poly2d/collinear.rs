use super::*;

pub fn collinear<T, U, V, W>(a: &U, b: &V, c: &W) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    W: AsV3d<T>,
    T: Float,
{
    area2(a, b, c).eps_eql(&T::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly2d_collinear() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];

        assert!(!collinear(o, a, b));
        assert!(!collinear(o, b, a));
        assert!(collinear(a, b, mid));
    }
}
