use super::*;

/// Reference: http://paulbourke.net/geometry/pointlineplane/
/// Line-Line Intersection
/// Closest Points: i
///   1. On Line_a: r1 = pa + s * va
///   2. On Line_b: r2 = pb + t * vb
/// Solve for s, t
pub fn intersect_line_line<T, U, V>(l1: &U, l2: &V) -> Option<[T; 3]>
where
    U: AsSegment<T>,
    V: AsSegment<T>,
    T: Float,
{
    if let Some((p1, p2)) = closest_points_between_lines(l1, l2) {
        if vector3d::eql(&p1, &p2) {
            return Some(p1);
        }
    }

    None
}

pub fn closest_points_between_lines<T, U, V>(l1: &U, l2: &V) -> Option<([T; 3], [T; 3])>
where
    U: AsSegment<T>,
    V: AsSegment<T>,
    T: Float,
{
    let pa = l1.to_p1();
    let va = l1.to_direction();

    let pb = l2.to_p1();
    let vb = l2.to_direction();

    let side = v3d!(pa - pb);

    let d_sa = vector3d::dot(&side, &va);
    let d_sb = vector3d::dot(&side, &vb);
    let d_ab = vector3d::dot(&va, &vb);

    let d_aa = vector3d::dot(&va, &va);
    let d_bb = vector3d::dot(&vb, &vb);

    let denom = d_aa * &d_bb - &(d_ab.clone() * &d_ab);

    if denom.eps_eql(&T::ZERO) {
        return None;
    }

    let numer = d_sb.clone() * &d_ab - &(d_sa * &d_bb);

    let s = numer / &denom;
    let t = (d_sb + &(d_ab * &s)) / &d_bb;

    let r1 = v3d!(pa + va .* s);
    let r2 = v3d!(pb + vb .* t);

    Some((r1, r2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_line_intersection() {
        let p1 = [12.15579466784725, 25.123635353834995, -84.00376833661478];
        let v1 = [-0.4250380747586038, 0.1362678492853246, 0.8948596025392205];
        let p2 = [-31.899763505672237, 56.530806846501484, -28.697077018204293];
        let v2 = [
            -0.26581664680768036,
            0.6654456941789392,
            -0.6975123929928632,
        ];

        let i = [-23.981999184105707, 36.70946669122478, -7.920580115491049];

        let a = Segment::from_point_vector(p1, v1);
        let b = Segment::from_point_vector(p2, v2);

        assert_eq!(intersect_line_line(&a, &b), Some(i));
    }

    #[test]
    fn test_segment_closest_points_between_lines() {
        let p1 = [12.0, 25.0, -84.0];
        let v1 = [-0.42, 0.13, 0.89];
        let p2 = [-31.0, 56.0, -28.0];
        let v2 = [-0.26, 0.66, -0.69];

        let r1 = [-23.919199871703235, 36.11784757933672, -7.885505033771707];
        let r2 = [-23.34251011845192, 36.5617564545318, -7.67819992973779];

        let a = Segment::from_point_vector(p1, v1);
        let b = Segment::from_point_vector(p2, v2);
        let res = closest_points_between_lines(&a, &b).unwrap();

        assert_eq!(res.0, r1);
        assert_eq!(res.1, r2);
    }
}
