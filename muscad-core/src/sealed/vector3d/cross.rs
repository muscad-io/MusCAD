use super::*;

/// unchecked addition
/// passing values with len < 3 causes panic

pub fn cross<U, V, T>(u: &U, v: &V) -> RtnType<T>
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Sub<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    [
        u[Y].clone() * &v[Z] - &(v[Y].clone() * &u[Z]),
        v[X].clone() * &u[Z] - &(u[X].clone() * &v[Z]),
        u[X].clone() * &v[Y] - &(v[X].clone() * &u[Y]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3d_cross() {
        let a = [3.0, 2.0, 5.0];
        let b = [1.0, 2.0, 3.0];
        let c = [-4.0, -4.0, 4.0];

        assert_eq!(vector3d::cross(&a, &b), c);
        assert_eq!(vector3d::cross(&b, &a), vector3d::reverse(&c));
    }
}
