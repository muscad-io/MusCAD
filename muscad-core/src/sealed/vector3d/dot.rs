use super::*;

pub fn dot<U, V, T>(u: &U, v: &V) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    u[X].clone() * &v[X] + &(u[Y].clone() * &v[Y]) + &(u[Z].clone() * &v[Z])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3d_dot() {
        let a = [0.89, 0.11, 0.04];
        let b = [0.99, 0.17, 0.48];
        let d = 0.919;

        assert_eq!(vector3d::dot(&a, &b), d);
    }
}
