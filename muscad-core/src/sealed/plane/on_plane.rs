use super::*;

pub fn on_plane<T, P, V>(pl: &V, pt: &P) -> bool
where
    V: AsPlane<T>,
    P: AsV3d<T>,
    T: Float,
{
    distance_to_point(pl, pt).eps_eql(&T::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_contains_pt() {
        let pl = [10.0, 0.0, 0.0, 100.0];

        let p1 = [10.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];
        let p3 = [-10.0, 8.0, 7.0];

        assert!(on_plane(&pl, &p1));
        assert!(on_plane(&pl, &p2));
        assert!(!on_plane(&pl, &p3));
    }
}
