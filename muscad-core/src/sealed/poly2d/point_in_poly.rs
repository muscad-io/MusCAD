/// Point in Polygon Test 2D
/// Reference: http://www.eecs.umich.edu/courses/eecs380/HANDOUTS/PROJ2/InsidePoly.html
use super::*;

pub fn point_in_poly_unchecked<V, P, PT, T>(point: &PT, poly: &V) -> bool
where
    V: AsRef<[P]>,
    P: AsV3d<T>,
    PT: AsV3d<T>,
    T: Float,
{
    let poly = poly.as_ref();
    let o = point.as_ref();

    let n = poly.len();
    let mut angle = T::zero();

    for i in 0..n {
        let p1 = poly[i].as_ref();
        let p2 = poly[(i + 1) % n].as_ref();

        angle += &vector2d::angle_between(&v2d!(p1 - o), &v2d!(p2 - o));
    }

    angle.abs() > T::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly2d_point_in_poly() {
        let poly = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];

        assert!(point_in_poly_unchecked(&[0.01, 0.01], &poly));
        assert!(!point_in_poly_unchecked(&[1.0, 0.01], &poly));
    }
}
