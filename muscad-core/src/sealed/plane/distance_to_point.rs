use super::*;

pub fn distance_to_point<T, P, V>(pl: &V, pt: &P) -> T
where
    V: AsPlane<T>,
    P: AsV3d<T>,
    T: Float,
{
    let n = pl.to_normal();
    let d = pl.d();

    let numer = vector3d::dot(&n, pt) - d;
    let denom = vector3d::length(&n);

    numer.abs() / &denom
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_distance_to_pt() {
        let pl = [1.0, 0.0, 0.0, 10.0];

        let p1 = [10.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];
        let p3 = [-10.0, 8.0, 7.0];

        assert_eq!(distance_to_point(&pl, &p1), 0.0);
        assert_eq!(distance_to_point(&pl, &p2), 0.0);
        assert_eq!(distance_to_point(&pl, &p3), 20.0);
    }
}
