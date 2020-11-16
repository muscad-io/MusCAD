use super::*;

/// unchecked addition
/// passing values with len < 3 causes panic

pub fn add<U, V, T>(u: &U, v: &V) -> RtnType<T>
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Add<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    [
        u[X].clone() + &v[X],
        u[Y].clone() + &v[Y],
        u[Z].clone() + &v[Z],
    ]
}

pub fn sub<U, V, T>(u: &U, v: &V) -> RtnType<T>
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    for<'a> T: Sub<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();
    let v = v.as_ref();

    [
        u[X].clone() - &v[X],
        u[Y].clone() - &v[Y],
        u[Z].clone() - &v[Z],
    ]
}

pub fn mul<U, T>(u: &U, s: &T) -> RtnType<T>
where
    U: AsV3d<T>,
    for<'a> T: Mul<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();

    [u[X].clone() * s, u[Y].clone() * s, u[Z].clone() * s]
}

pub fn div<U, T>(u: &U, s: &T) -> RtnType<T>
where
    U: AsV3d<T>,
    for<'a> T: Div<&'a T, Output = T> + Clone,
{
    let u = u.as_ref();

    [u[X].clone() / s, u[Y].clone() / s, u[Z].clone() / s]
}

pub fn add_mut<U, V, T>(u: &mut U, v: &V)
where
    U: AsV3dMut<T>,
    V: AsV3d<T>,
    for<'a> T: AddAssign<&'a T>,
{
    let u = u.as_mut();
    let v = v.as_ref();

    u[X] += &v[X];
    u[Y] += &v[Y];
    u[Z] += &v[Z];
}

pub fn sub_mut<U, V, T>(u: &mut U, v: &V)
where
    U: AsV3dMut<T>,
    V: AsV3d<T>,
    for<'a> T: SubAssign<&'a T>,
{
    let u = u.as_mut();
    let v = v.as_ref();

    u[X] -= &v[X];
    u[Y] -= &v[Y];
    u[Z] -= &v[Z];
}

pub fn mul_mut<U, T>(u: &mut U, s: &T)
where
    U: AsV3dMut<T>,
    for<'a> T: MulAssign<&'a T>,
{
    let u = u.as_mut();

    u[X] *= &s;
    u[Y] *= &s;
    u[Z] *= &s;
}

pub fn div_mut<U, T>(u: &mut U, s: &T)
where
    U: AsV3dMut<T>,
    for<'a> T: DivAssign<&'a T>,
{
    let u = u.as_mut();

    u[X] /= &s;
    u[Y] /= &s;
    u[Z] /= &s;
}

pub fn assign<U, V, T>(out: &mut U, v: &V)
where
    U: AsV3dMut<T>,
    V: AsV3d<T>,
    T: Float,
{
    let u = out.as_mut();
    let v = v.as_ref();

    u[X] = v[X].clone();
    u[Y] = v[Y].clone();
    u[Z] = v[Z].clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3d_assign() {
        let mut u = [1.0, 2.0, 3.0];
        let v = [2.0, 3.0, 4.0];

        vector3d::assign(&mut u, &v);

        assert_eq!(u, v);
    }

    #[test]
    fn test_v3d_add() {
        let u = [1.0, 2.0, 3.0];
        let v = [2.0, 3.0, 4.0];

        let r = vector3d::add(&u, &v);

        assert_eq!(r, [3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_v3d_add_mut() {
        let mut u = [1.0, 2.0, 3.0];
        let v = [2.0, 3.0, 4.0];

        vector3d::add_mut(&mut u, &v);

        assert_eq!(u, [3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_v3d_sub() {
        let u = [1.0, 2.0, 3.0];
        let v = [2.0, 3.0, 4.0];

        let r = vector3d::sub(&u, &v);

        assert_eq!(r, [-1.0, -1.0, -1.0]);
    }

    #[test]
    fn test_v3d_sub_mut() {
        let mut u = [1.0, 2.0, 3.0];
        let v = [2.0, 3.0, 4.0];

        vector3d::sub_mut(&mut u, &v);

        assert_eq!(u, [-1.0, -1.0, -1.0]);
    }

    #[test]
    fn test_v3d_mul() {
        let u = [1.0, 2.0, 3.0];
        let v = 2.0;

        let r = vector3d::mul(&u, &v);

        assert_eq!(r, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_v3d_mul_mut() {
        let mut u = [1.0, 2.0, 3.0];
        let v = 2.0;

        vector3d::mul_mut(&mut u, &v);

        assert_eq!(u, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_v3d_div() {
        let u = [1.0, 2.0, 3.0];
        let v = 2.0;

        let r = vector3d::div(&u, &v);

        assert_eq!(r, [0.5, 1.0, 1.5]);
    }

    #[test]
    fn test_v3d_div_mut() {
        let mut u = [1.0, 2.0, 3.0];
        let v = 2.0;

        vector3d::div_mut(&mut u, &v);

        assert_eq!(u, [0.5, 1.0, 1.5]);
    }
}
