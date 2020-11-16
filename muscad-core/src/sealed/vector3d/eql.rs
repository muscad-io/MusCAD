use super::*;

pub fn eql<U, V, T>(u: &U, v: &V) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T:
        Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Sub<&'a T, Output = T> + Clone + Float,
{
    let d = distance_sq(u, v);

    d.lt_eps_sq()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3d_eql() {
        let zero = vector3d::zero();

        let v1 = [1.0, 1.0, 1.0];
        let v2 = [2.0, 2.0, 2.0];
        let v3 = [3.0, 3.0, 3.0];

        assert!(vector3d::eql(&v3, &vector3d::add(&v1, &v2)));
        assert!(vector3d::eql(&[0.0, 0.0, 0.0], &zero));
        assert!(!vector3d::eql(&[0.0, 0.01, 0.0], &zero));
    }
}
