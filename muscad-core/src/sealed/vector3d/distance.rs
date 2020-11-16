use super::*;

pub fn distance<U, V, T>(u: &U, v: &V) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T:
        Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Sub<&'a T, Output = T> + Clone + Float,
{
    distance_sq(u, v).sqrt()
}

pub fn distance_sq<U, V, T>(u: &U, v: &V) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Sub<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    let x = u[X].clone() - &v[X];
    let y = u[Y].clone() - &v[Y];
    let z = u[Z].clone() - &v[Z];

    x.clone() * &x + &(y.clone() * &y) + &(z.clone() * &z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3d_distance_sq() {
        let u = [1.0, 2.0, 3.0];
        let v = [11.0, -2.0, 30.0];

        let d2 = 845.0;

        assert_eq!(vector3d::distance_sq(&u, &v), d2);
    }

    #[test]
    fn test_v3d_distance() {
        let u = [1.0, 2.0, 3.0];
        let v = [11.0, -2.0, 30.0];

        let d = 29.068883707497267;

        assert_eq!(vector3d::distance(&u, &v), d);
    }
}
