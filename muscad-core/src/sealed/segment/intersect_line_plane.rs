use super::*;

/// Line-Plane Intersection
/// Line (p1, p2)
/// Line Direction: dir = p2 - p1
/// Plane Eq: dot(plane.normal, pt) == plane.d
/// Intersection: i
///   1. On Plane: dot(plane.normal, i) == plane.d
///   2. On Line: i = p1 + t*dir
/// Solve: dot(n, p1 + t*dir) == d
///        t = (d - n*p1) / n*dir

pub fn intersect_line_plane<T, U, V>(ln: &U, pl: &V) -> Option<[T; 3]>
where
    U: AsSegment<T>,
    V: AsPlane<T>,
    T: Float,
{
    let p1 = &ln.to_p1();
    let v = ln.to_direction();
    let n = pl.to_normal();

    let d_nv = vector3d::dot(&v, &n);

    if d_nv.eps_eql(&T::ZERO) {
        return None;
    }

    let d_np = vector3d::dot(p1, &n);

    let t = (pl.d().clone() - &d_np) / &d_nv;

    Some(v3d!(p1 + v .* t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_intersect_line_plane() {
        let pl = [0.74, 0.80, 0.12, 0.21];
        let ln = Segment::from_point_point([1.0, 2.0, 3.0], [33.0, -2.0, 1.0]);

        let r = intersect_line_plane(&ln, &pl).unwrap();
        let i = [-2.9367588932806314, 2.492094861660079, 3.2460474308300395];

        assert_eq!(r, i);
    }
}
