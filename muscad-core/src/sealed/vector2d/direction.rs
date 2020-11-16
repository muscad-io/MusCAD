use super::*;

fn _is_parallel<U, V, T>(u: &U, v: &V) -> (T, bool)
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    T: Float,
{
    let d = dot(u, v);

    let is_para = (d.clone() * &d - &(length_sq(u) * &length_sq(v)))
        .abs()
        .lt_eps_sq();

    (d, is_para)
}

#[allow(unused)]
fn is_parallel<U, V, T>(u: &U, v: &V) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    T: Float,
{
    _is_parallel(u, v).1
}

pub fn is_same_direction<U, V, T>(u: &U, v: &V) -> bool
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    T: Float,
{
    let (dp, is_para) = _is_parallel(u, v);

    is_para && dp.eps_gte(&T::ZERO)
}

pub fn reverse<U, T>(u: &U) -> RtnType<T>
where
    U: AsV3d<T>,
    T: Float,
{
    let u = u.as_ref();

    [-u[X].clone(), -u[Y].clone()]
}

pub fn reverse_mut<U, T>(u: &mut U)
where
    U: AsV3dMut<T>,
    T: Float,
{
    let u = u.as_mut();
    u[X] = -u[X].clone();
    u[Y] = -u[Y].clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_reverse() {
        let u = [1.0, 2.0];

        assert_eq!(vector2d::reverse(&u), [-1.0, -2.0]);
    }

    #[test]
    fn test_v2d_reverse_mut() {
        let mut u = [1.0, 2.0];

        vector2d::reverse_mut(&mut u);

        assert_eq!(u, [-1.0, -2.0]);
    }

    #[test]
    fn test_v2d_is_parallel() {
        let u = [1.0, 2.0];
        let v = [-1.0, -2.0];
        let w = [-2.0, -4.0];

        assert!(is_parallel(&u, &v));
        assert!(is_parallel(&u, &w));
        assert!(is_parallel(&u, &vector2d::zero()));
        assert!(is_parallel(&[0.0, 0.0], &vector2d::zero()));
        assert!(!is_parallel(&u, &[f64::PI, 1.0]));
    }

    #[test]
    fn test_v2d_is_same_direction() {
        let u = [1.0, 2.0];
        let v = [-1.0, -2.0];
        let w = [-2.0, -4.0];

        assert!(!is_same_direction(&u, &v));
        assert!(!is_same_direction(&u, &w));
        assert!(is_same_direction(&v, &w));
        assert!(is_same_direction(&u, &vector2d::zero()));
        assert!(is_same_direction(&[0.0, 0.0], &vector2d::zero()));
        assert!(!is_same_direction(&u, &[f64::PI, 1.0]));
    }
}
