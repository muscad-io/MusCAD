use super::*;

/// test whether c is on segment ab. Endpoints are included.
pub fn between<T, U, V, W>(a: &U, b: &V, c: &W) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    W: AsV3d<T>,
    T: Float,
{
    if !collinear(a, b, c) {
        return false;
    }

    let a = a.as_ref();
    let b = b.as_ref();
    let c = c.as_ref();

    if a[X].eps_eql(&b[X]) {
        _in_range(&a[Y], &b[Y], &c[Y])
    } else {
        _in_range(&a[X], &b[X], &c[X])
    }
}

fn _in_range<T: Float>(r1: &T, r2: &T, v: &T) -> bool {
    (r1.eps_lte(v) && v.eps_lte(r2)) || (r2.eps_lte(v) && v.eps_lte(r1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly2d_between() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];

        assert!(between(a, b, mid));
        assert!(between(a, b, b));
        assert!(between(a, b, a));
        assert!(!between(a, b, o));
    }
}
