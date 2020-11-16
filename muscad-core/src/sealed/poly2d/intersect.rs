use super::*;

/// test whether two segments intersect. Intersection at endpoints returns false.
pub fn prop_intersect<T, U, V, X, Y>(a: &U, b: &V, c: &X, d: &Y) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    X: AsV3d<T>,
    Y: AsV3d<T>,
    T: Float,
{
    if collinear(a, b, c) || collinear(a, b, d) || collinear(c, d, a) || collinear(c, d, b) {
        false
    } else {
        left(a, b, c) ^ left(a, b, d) && left(c, d, a) ^ left(c, d, b)
    }
}

/// test whether two segments intersect. Handles collinear cases.
pub fn intersect<T, U, V, X, Y>(a: &U, b: &V, c: &X, d: &Y) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    X: AsV3d<T>,
    Y: AsV3d<T>,
    T: Float,
{
    if prop_intersect(a, b, c, d) {
        true
    } else if between(a, b, c) || between(a, b, d) || between(c, d, a) || between(c, d, b) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //      b
    //    / | \
    //   /  |  \
    // na---o---a
    //   \  |  /
    //    \ | /
    //      nb
    #[test]
    fn test_poly2d_intersect() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];
        let na = &[-1.0, 0.0];
        let nb = &[0.0, -1.0];

        assert!(intersect(a, na, b, nb));
        assert!(intersect(o, na, b, nb));
        assert!(intersect(o, na, o, a));
        assert!(!intersect(mid, o, na, nb));
    }

    //      b
    //    / | \
    //   /  |  \
    // na---o---a
    //   \  |  /
    //    \ | /
    //      nb
    #[test]
    fn test_poly2d_prop_intersect() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];
        let na = &[-1.0, 0.0];
        let nb = &[0.0, -1.0];

        assert!(prop_intersect(a, na, b, nb));
        assert!(!prop_intersect(o, na, b, nb));
        assert!(!prop_intersect(mid, o, na, nb));
    }
}
