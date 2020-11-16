use super::*;

pub fn cross<U, V, T>(u: &U, v: &V) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Sub<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    u[X].clone() * &v[Y] - &(v[X].clone() * &u[Y])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_cross() {
        let a = [3.0, 2.0];
        let b = [1.0, 2.0];

        let c = 4.0;

        assert_eq!(vector2d::cross(&a, &b), c);
        assert_eq!(vector2d::cross(&b, &a), -c);
    }
}
