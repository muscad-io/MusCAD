use super::*;

pub fn left<T, U, V, W>(a: &U, b: &V, c: &W) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    W: AsV3d<T>,
    T: Float,
{
    area2(a, b, c).eps_gt(&T::ZERO)
}

pub fn left_on<T, U, V, W>(a: &U, b: &V, c: &W) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    W: AsV3d<T>,
    T: Float,
{
    area2(a, b, c).eps_gte(&T::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    // b
    // | \  mid
    // |  \
    // o---a
    #[test]
    fn test_poly2d_left() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];

        assert!(left(o, a, b));
        assert!(!left(o, b, a));
        assert!(!left(a, b, mid));
    }

    #[test]
    fn test_poly2d_left_on() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];

        assert!(left_on(o, a, b));
        assert!(!left_on(o, b, a));
        assert!(left_on(a, b, mid));
    }
}
