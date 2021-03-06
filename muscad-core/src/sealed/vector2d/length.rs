use super::*;

pub fn length_sq<U, T>(u: &U) -> T
where
    U: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();

    u[X].clone() * &u[X] + &(u[Y].clone() * &u[Y])
}

pub fn length<U, T>(u: &U) -> T
where
    U: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone + Float,
{
    length_sq(u).sqrt()
}

pub fn is_zero<U, T>(u: &U) -> bool
where
    U: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + Clone + Float,
{
    let d = length_sq(u);

    d.lt_eps_sq()
}

pub fn normalize<U, T>(u: &U) -> RtnType<T>
where
    U: AsV3d<T>,
    T: Float,
{
    let l = length(u);

    div(u, &l)
}

pub fn normalize_mut<U, T>(u: &mut U)
where
    U: AsV3dMut<T>,
    for<'a> T: Add<&'a T, Output = T> + Mul<&'a T, Output = T> + DivAssign<&'a T> + Clone + Float,
{
    let l = length(&u.as_mut());

    div_mut(u, &l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_length_sq() {
        let u = [1.0, 2.0];
        let r = vector2d::length_sq(&u);

        assert_eq!(r, 5.0);
    }

    #[test]
    fn test_v2d_length() {
        let u = [1.0, 2.0];
        let r = vector2d::length(&u);

        assert_eq!(r, 5.0.sqrt());
    }

    #[test]
    fn is_zero() {
        assert!(vector2d::is_zero(&[0.0, 0.0]));
        assert!(!vector2d::is_zero(&[0.0, 0.1]));
    }

    #[test]
    fn test_v2d_normalize_mut() {
        let mut u = [1.0, 2.0];
        let l = 5.0.sqrt();

        vector2d::normalize_mut(&mut u);

        assert_eq!(u, [1.0 / l, 2.0 / l]);
    }

    #[test]
    fn test_v2d_normalize() {
        let u = [1.0, 2.0];
        let l = 5.0.sqrt();

        let r = vector2d::normalize(&u);

        assert_eq!(r, [1.0 / l, 2.0 / l]);
    }
}
