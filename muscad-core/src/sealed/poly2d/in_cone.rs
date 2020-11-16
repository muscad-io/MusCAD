use super::*;

/// test whether p lies strictly in cone formed by oa to ob.
pub fn in_cone<T, O, A, B, P>(o: &O, a: &A, b: &B, p: &P) -> bool
where
    O: AsV3d<T>,
    A: AsV3d<T>,
    B: AsV3d<T>,
    P: AsV3d<T>,
    T: Float,
{
    if _cone_convex(o, a, b) {
        //  b   p
        //  |
        //  o-----a
        left(o, p, b) && left(p, o, a)
    } else {
        //    b
        //    |
        // p  o-----a
        !(left_on(o, p, a) && left_on(p, o, b))
    }
}

fn _cone_convex<T, O, A, B>(o: &O, a: &A, b: &B) -> bool
where
    O: AsV3d<T>,
    A: AsV3d<T>,
    B: AsV3d<T>,
    T: Float,
{
    left_on(o, a, b)
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
    fn test_poly2d_in_cone() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];
        let na = &[-1.0, 0.0];
        let nb = &[0.0, -1.0];
        let b2 = &[0.0, 2.0];

        assert!(in_cone(o, a, b, mid));
        assert!(!in_cone(o, a, b, o));
        assert!(!in_cone(o, a, b, a));
        assert!(!in_cone(o, a, b, b));

        assert!(!in_cone(b, a, na, o));
        assert!(!in_cone(b, a, na, b));
        assert!(!in_cone(b, a, na, a));
        assert!(!in_cone(b, a, na, na));
        assert!(in_cone(b, a, na, b2));

        assert!(!in_cone(nb, na, a, b2));
    }
}
