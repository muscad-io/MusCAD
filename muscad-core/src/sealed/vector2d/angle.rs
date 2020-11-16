use super::*;

pub fn angle_between<U, V, T>(u: &U, v: &V) -> T
where
    U: AsV3d<T>,
    V: AsV3d<T>,
    T: Float,
{
    let u = u.as_ref();
    let v = v.as_ref();

    let a1 = u[Y].clone().atan2(u[X].clone());
    let mut a2 = v[Y].clone().atan2(v[X].clone());

    a2 -= &a1;

    if a2 > T::PI {
        a2 - &T::TAU
    } else if a2 <= -T::PI {
        a2 + &T::TAU
    } else {
        a2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_angle_between() {
        let v1 = [1.0, 0.0];
        let v2 = [0.0, 1.0];
        let v3 = [0.0, -1.0];

        assert_eq!(angle_between(&v1, &v2), f64::PI / 2.0);
        assert_eq!(angle_between(&v1, &v3), -f64::PI / 2.0);
        assert_eq!(angle_between(&[-1.0, 1.0], &[-1.0, -1.0]), f64::PI / 2.0);
    }
}
