use super::*;

pub fn area2<T, U, V, W>(a: &U, b: &V, c: &W) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    W: AsV3d<T>,
    T: Float,
{
    v2d!((b - a) * (c - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly2d_area2() {
        let o = &[0.0, 0.0];
        let a = &[1.0, 0.0];
        let b = &[0.0, 1.0];

        let mid = &[0.5, 0.5];

        assert_eq!(area2(o, a, b), 1.0);
        assert_eq!(area2(o, b, a), -1.0);
        assert_eq!(area2(mid, a, b), 0.0);
    }
}
