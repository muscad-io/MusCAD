/// Project polygons from 3D Space to 2D Space without losing topological properties.
/// Six Options:
/// project wrt +X_AXIS --> Y_AXIS x Z_AXIS == +X_AXIS
/// project wrt -X_AXIS --> Z_AXIS x Y_AXIS == -X_AXIS
/// project wrt +Y_AXIS --> Z_AXIS x X_AXIS == +Y_AXIS
/// project wrt -Y_AXIS --> X_AXIS x Z_AXIS == -Y_AXIS
/// project wrt +Z_AXIS --> X_AXIS x Y_AXIS == +Z_AXIS
/// project wrt -Z_AXIS --> Y_AXIS x X_AXIS == -Z_AXIS
use super::*;

pub fn project_x<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[Y].clone(), v[Z].clone()]
}

pub fn project_neg_x<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[Z].clone(), v[Y].clone()]
}

pub fn project_y<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[Z].clone(), v[X].clone()]
}

pub fn project_neg_y<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[X].clone(), v[Z].clone()]
}

pub fn project_z<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[X].clone(), v[Y].clone()]
}

pub fn project_neg_z<V, T>(v: &V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Clone,
{
    let v = v.as_ref();

    [v[Y].clone(), v[X].clone()]
}

pub fn to_functor<V, T>(a: &T, b: &T, c: &T) -> for<'r> fn(&'r V) -> [T; 2]
where
    V: AsV3d<T>,
    T: Float,
{
    let x = a.clone().abs();
    let y = b.clone().abs();
    let z = c.clone().abs();

    if x > y && x > z {
        if a > &T::ZERO {
            project_x
        } else {
            project_neg_x
        }
    } else if y > x && y > z {
        if b > &T::ZERO {
            project_y
        } else {
            project_neg_y
        }
    } else {
        if c > &T::ZERO {
            project_z
        } else {
            project_neg_z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project2d_to_functor() {
        let v = [1.0, 2.0, 3.0];

        assert_eq!((to_functor(&1.0, &2.0, &3.0))(&v), [v[X], v[Y]]);
        assert_eq!((to_functor(&1.0, &2.0, &-3.0))(&v), [v[Y], v[X]]);

        assert_eq!((to_functor(&1.0, &22.0, &3.0))(&v), [v[Z], v[X]]);
        assert_eq!((to_functor(&1.0, &-22.0, &3.0))(&v), [v[X], v[Z]]);

        assert_eq!((to_functor(&10.0, &0.0, &3.0))(&v), [v[Y], v[Z]]);
        assert_eq!((to_functor(&-10.0, &0.0, &3.0))(&v), [v[Z], v[Y]]);
    }
}
