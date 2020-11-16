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

    x.clone() * &x + &(y.clone() * &y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_distance_sq() {
        let u = [10.0, 10.0];
        let v = [13.0, 14.0];

        let d2 = 25.0;

        assert_eq!(vector2d::distance_sq(&u, &v), d2);
    }

    #[test]
    fn test_v2d_distance() {
        let u = [10.0, 10.0];
        let v = [13.0, 14.0];

        let d = 5.0;

        assert_eq!(vector2d::distance(&u, &v), d);
    }
}
