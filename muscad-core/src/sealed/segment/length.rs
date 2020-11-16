use super::*;

pub fn length<T, S>(ln: &S) -> T
where
    S: AsSegment<T>,
    T: Float,
{
    vector3d::length(&ln.to_direction())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_length() {
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];

        let ln = Segment::from_point_point(p1.clone(), p2.clone());

        assert_eq!(length(&ln), 194.0.sqrt());
    }
}
