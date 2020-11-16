use super::*;

pub fn on_segment<T, S, P>(ln: &S, pt: &P) -> (bool, bool, bool)
where
    S: AsSegment<T>,
    P: AsV3d<T>,
    T: Float,
{
    let p1 = &ln.to_p1();
    let p2 = &ln.to_p2();

    if vector3d::eql(p1, pt) {
        return (true, true, false);
    }

    if vector3d::eql(p2, pt) {
        return (true, false, true);
    }

    let d = vector3d::distance(p1, pt) + &vector3d::distance(p2, pt);

    (d.eps_eql(&length(ln)), false, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_segment_on_segment() {
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];
        let v = [9.0, 8.0, 7.0];

        let ln = Segment::from_point_point(p1.clone(), p2.clone());

        let t1 = 0.333333;
        let pt1 = v3d!(p1 + v .* t1);

        let t2 = -0.333333;
        let pt2 = v3d!(p1 + v .* t2);

        let t3 = 0.8;
        let pt3 = v3d!(p1 + v .* t3);

        assert_eq!(on_segment(&ln, &p1), (true, true, false));
        assert_eq!(on_segment(&ln, &p2), (true, false, true));

        assert_eq!(on_segment(&ln, &pt1), (true, false, false));
        assert_eq!(on_segment(&ln, &pt2), (false, false, false));
        assert_eq!(on_segment(&ln, &pt3), (true, false, false));
    }
}
